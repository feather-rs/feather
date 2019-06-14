use bytes::{Buf, BufMut};
use super::mctypes::{McTypeWrite, McTypeRead};
use super::packet;
use super::packet::{Packet, PacketStage, PacketDirection, PacketType, PacketId};
use openssl::symm::{Cipher, Crypter, Mode};
use flate2::{read::{ZlibDecoder, ZlibEncoder}, Compression};
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use crate::bytebuf::ByteBuf;

pub type PacketAcceptor = fn(Box<Packet>) -> ();

pub struct ConnectionIOManager {
    encryption_enabled: bool,
    encryption_key: [u8; 16],
    compression_enabled: bool,
    compression_threshold: i32,
    packet_acceptor: PacketAcceptor,

    incoming_compressed: ByteBuf,
    incoming_uncompressed: ByteBuf,

    encrypter: Option<Crypter>,
    decrypter: Option<Crypter>,

    stage: PacketStage,

    cipher: Cipher,
}

impl ConnectionIOManager {
    pub fn new(packet_acceptor: PacketAcceptor) -> Self {
        Self {
            encryption_enabled: false,
            encryption_key: [0; 16],
            compression_enabled: false,
            compression_threshold: -1,
            packet_acceptor,

            incoming_compressed: ByteBuf::new(),
            incoming_uncompressed: ByteBuf::new(),

            encrypter: None,
            decrypter: None,

            stage: PacketStage::Handshake,

            cipher: Cipher::aes_128_cfb8(),
        }
    }

    pub fn set_stage(&mut self, stage: PacketStage) {
        self.stage = stage;
    }

    pub fn enable_encryption(&mut self, key: [u8; 16]) {
        self.encryption_enabled = true;
        self.encryption_key = key;

        self.encrypter = Some(Crypter::new(
            self.cipher,
            Mode::Encrypt,
            &key,
            Some(&key)).unwrap());
        self.decrypter = Some(Crypter::new(
            self.cipher,
            Mode::Decrypt,
            &key,
            Some(&key)).unwrap());
    }

    pub fn enable_compression(&mut self, threshold: i32) {
        self.compression_enabled = true;
        self.compression_threshold = threshold;
    }

    /// `Err` is returned only if something happens that indicates
    /// a malicious client. If `Err` is returned, the client should
    /// be disconnected immediately.
    pub fn accept_data(&mut self, data: &mut ByteBuf) -> Result<(), ()> {
        // Decrypt if needed
        if self.encryption_enabled {
            self.decrypt_data(data.inner());
        } else {
            // Copy to incoming_compressed without decrypting
            self.incoming_compressed.put(data.inner());
        }

        data.remove_prior();

        // Mark reader index so we can return to this
        // position in the buffer if the packet is incomplete
        let buf = &mut self.incoming_compressed;
        buf.mark_read_position();

        // Read packet length field - return to wait for more data if failed
        let packet_length = {
            if let Ok(len) = buf.read_var_int() {
                len
            } else {
                buf.reset_read_position();
                return Ok(());
            }
        };

        // Check that the entire packet is received - otherwise, return and
        // wait for more bytes
        if (buf.remaining() as i32) < packet_length {
            buf.reset_read_position();
            return Ok(());
        }

        // If compression is enabled, read the uncompressed length
        // and decompress - otherwise, copy bytes to incoming_uncompressed
        if self.compression_enabled {
            let uncompressed_size = buf.read_var_int()?;
            self.decompress_data(uncompressed_size);
        } else {
            self.incoming_uncompressed.put(buf.inner());
        }

        self.incoming_compressed.remove_prior();

        let buf = &mut self.incoming_uncompressed;

        let packet_id = buf.read_var_int()?;
        let stage = self.stage;
        let direction = PacketDirection::Serverbound; // Have to change to implement client

        let packet_type = PacketType::get_from_id(PacketId(packet_id as u32, direction, stage));
        if packet_type.is_err() {
            warn!("Client sent packet with invalid id {} for stage {:?}", packet_id, stage);
        }

        let mut packet = packet_type.unwrap().get_implementation();
        packet.read_from(buf)?;

        buf.remove_prior();

        let f = self.packet_acceptor;
        f(packet);

        Ok(())
    }

    pub fn serialize_packet(&mut self, packet: Box<Packet>) -> ByteBuf {
        let mut packet_data_buf = ByteBuf::new();
        packet.write_to(&mut packet_data_buf);

        let mut buf_without_length = ByteBuf::with_capacity(packet_data_buf.len());

        if self.compression_enabled {
            let mut uncompressed_length = packet_data_buf.len();

            if packet.len() < self.compression_threshold {
                uncompressed_length = 0;
                buf_without_length.write_var_int(0);
                buf_without_length.put(packet_data_buf);
            } else {
                buf_without_length.write_var_int(uncompressed_length as i32);
                self.compress_data(packet_data_buf.inner(), &mut buf_without_length);
            }
        }

        let mut buf = ByteBuf::with_capacity(buf_without_length + 4);
        buf.write_var_int(buf_without_length.len() as i32);
        buf.put(buf_without_length.inner());

        buf
    }

    fn encrypt_data(&mut self, data: &[u8], output: &mut ByteBuf) {
        let crypter = self.encrypter.as_mut().unwrap();

        let needed_bytes = self.cipher.block_size() + data.len();

        output.reserve(needed_bytes);

        crypter.update(data, output.inner_mut()).unwrap();
    }

    fn decrypt_data(&mut self, data: &[u8]) {
        let crypter = self.decrypter.as_mut().unwrap();

        let output = &mut self.incoming_compressed;

        let needed_bytes = self.cipher.block_size() + data.len();
        output.reserve(needed_bytes);

        crypter.update(data, output.inner_mut()).unwrap();
    }

    fn compress_data(&mut self, data: &[u8], output: &mut ByteBuf) {
        let mut coder = ZlibEncoder::new(data, Compression::default());
        output.reserve(coder.total_out() as usize);
        coder.read(output.inner_mut()).unwrap();
    }

    fn decompress_data(&mut self, uncompressed_size: i32) {
        let data = &mut self.incoming_compressed;
        if uncompressed_size == 0 {
            self.incoming_uncompressed.reserve(data.len());
            self.incoming_uncompressed.put(data.inner());
        }
        let mut coder = ZlibDecoder::new(data);
        self.incoming_uncompressed.reserve(uncompressed_size as usize);
        coder.read(self.incoming_uncompressed.inner_mut()).unwrap();
    }
}