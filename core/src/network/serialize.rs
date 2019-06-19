use super::mctypes::{McTypeRead, McTypeWrite};
use super::packet::{Packet, PacketDirection, PacketId, PacketStage, PacketType};
use crate::bytebuf::{BufMutAlloc, ByteBuf};
use crate::prelude::*;
use bytes::{Buf, BufMut};
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use openssl::symm::{Cipher, Crypter, Mode};
use std::io::prelude::*;
use std::io::Cursor;

pub struct ConnectionIOManager {
    encryption_enabled: bool,
    encryption_key: [u8; 16],
    compression_enabled: bool,
    compression_threshold: usize,

    pending_received_packets: Option<Vec<Box<Packet>>>,

    incoming_compressed: ByteBuf,
    incoming_uncompressed: ByteBuf,

    encrypter: Option<Crypter>,
    decrypter: Option<Crypter>,

    stage: PacketStage,

    cipher: Cipher,
}

impl ConnectionIOManager {
    pub fn new() -> Self {
        Self {
            encryption_enabled: false,
            encryption_key: [0; 16],
            compression_enabled: false,
            compression_threshold: 0,
            pending_received_packets: Some(vec![]),

            incoming_compressed: ByteBuf::with_capacity(128),
            incoming_uncompressed: ByteBuf::with_capacity(128),

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

        self.encrypter = Some(Crypter::new(self.cipher, Mode::Encrypt, &key, Some(&key)).unwrap());
        self.decrypter = Some(Crypter::new(self.cipher, Mode::Decrypt, &key, Some(&key)).unwrap());

        trace!("Enabling encryption");
    }

    pub fn enable_compression(&mut self, threshold: usize) {
        self.compression_enabled = true;
        self.compression_threshold = threshold;

        trace!("Enabling compression");
    }

    /// `Err` is returned only if something happens that indicates
    /// a malicious client. If `Err` is returned, the client should
    /// be disconnected immediately.
    pub fn accept_data(&mut self, data: ByteBuf) -> Result<(), ()> {
        // Decrypt if needed
        if self.encryption_enabled {
            self.decrypt_data(data.inner());
        } else {
            // Copy to incoming_compressed without decrypting
            self.incoming_compressed.write(data.inner());
        }

        loop {
            let pending_buf = &mut self.incoming_compressed;

            // Mark reader index so we can return to this
            // position in the buffer if the packet is incomplete
            pending_buf.mark_read_position();

            let packet_length = {
                if let Ok(val) = pending_buf.read_var_int() {
                    val
                } else {
                    pending_buf.reset_read_position();
                    break;
                }
            };

            // Check that the entire packet is received - otherwise, return and
            // wait for more bytes
            if (pending_buf.remaining() as i32) < packet_length {
                pending_buf.reset_read_position();
                return Ok(());
            }

            pending_buf.mark_read_position();

            // If compression is enabled, read the uncompressed length
            // and decompress - otherwise, copy bytes to incoming_uncompressed
            let len_of_compressed_size_field;
            if self.compression_enabled {
                let uncompressed_size = pending_buf.read_var_int()?;
                if uncompressed_size != 0 {
                    self.decompress_data(uncompressed_size);
                    len_of_compressed_size_field = 0;
                } else {
                    self.incoming_uncompressed
                        .write(&pending_buf.inner()[..(packet_length - 1) as usize]);
                    len_of_compressed_size_field =
                        pending_buf.read_pos() - pending_buf.marked_read_position();
                    pending_buf.advance((packet_length - 1) as usize);
                }
            } else {
                len_of_compressed_size_field = 0;
                let buf = &pending_buf.inner()[..(packet_length as usize)];
                self.incoming_uncompressed.write(buf);
                self.incoming_compressed.advance(packet_length as usize);
            }

            self.incoming_compressed.remove_prior();

            let buf = &mut self.incoming_uncompressed;
            buf.mark_read_position();

            let packet_id = buf.read_var_int()?;
            let stage = self.stage;
            let direction = PacketDirection::Serverbound; // Have to change to implement client

            let packet_type = PacketType::get_from_id(PacketId(packet_id as u32, direction, stage));
            if packet_type.is_err() {
                warn!(
                    "Client sent packet with invalid id {} for stage {:?}",
                    packet_id, stage
                );

                return Err(());
            }

            trace!("Received packet with type {:?}", packet_type.unwrap());

            let mut packet = packet_type.unwrap().get_implementation();
            let upper_index = packet_length as usize
                - (buf.read_pos() - buf.marked_read_position())
                - len_of_compressed_size_field;
            {
                let mut slice = Cursor::new(&buf.inner()[..upper_index]);
                packet.read_from(&mut slice)?;
            }
            buf.advance(upper_index);

            if packet.ty() == PacketType::Handshake {
                let handshake =
                    cast_packet::<crate::network::packet::implementation::Handshake>(&packet);
                match handshake.next_state {
                    crate::network::packet::implementation::HandshakeState::Login => {
                        self.stage = PacketStage::Login
                    }
                    crate::network::packet::implementation::HandshakeState::Status => {
                        self.stage = PacketStage::Status
                    }
                }
            }

            buf.remove_prior();

            self.pending_received_packets.as_mut().unwrap().push(packet);
        }

        Ok(())
    }

    pub fn serialize_packet(&mut self, packet: Box<Packet>) -> ByteBuf {
        if packet.ty() == PacketType::LoginSuccess {
            self.stage = PacketStage::Play;
        }

        trace!("Sending packet with type {:?}", packet.ty());

        let mut packet_data_buf = ByteBuf::with_capacity(16);
        packet_data_buf.write_var_int(packet.ty().get_id().0 as i32);
        packet.write_to(&mut packet_data_buf);

        let mut buf_without_length = ByteBuf::with_capacity(packet_data_buf.len());

        if self.compression_enabled {
            let uncompressed_length = packet_data_buf.len();

            if packet_data_buf.len() < self.compression_threshold as usize {
                buf_without_length.write_var_int(0);
                buf_without_length.write(packet_data_buf.inner());
            } else {
                buf_without_length.write_var_int(uncompressed_length as i32);
                self.compress_data(packet_data_buf.inner(), &mut buf_without_length);
            }
        } else {
            buf_without_length.write(packet_data_buf.inner()); // Lots of inefficient copying here - find a fix for this
        }

        let mut buf = ByteBuf::with_capacity(buf_without_length.len() + 4);
        buf.write_var_int(buf_without_length.len() as i32);
        buf.write(buf_without_length.inner());

        if !self.encryption_enabled {
            buf
        } else {
            let mut encrypted_buf = ByteBuf::with_capacity(buf.len());
            self.encrypt_data(buf.inner(), &mut encrypted_buf);
            encrypted_buf
        }
    }

    fn encrypt_data(&mut self, data: &[u8], output: &mut ByteBuf) {
        let crypter = self.encrypter.as_mut().unwrap();

        let needed_bytes = self.cipher.block_size() + data.len();

        output.reserve(needed_bytes);

        unsafe {
            let amnt = crypter.update(data, output.inner_mut()).unwrap();
            output.advance_mut(amnt);
        }
    }

    fn decrypt_data(&mut self, data: &[u8]) {
        let crypter = self.decrypter.as_mut().unwrap();

        let output = &mut self.incoming_compressed;

        let needed_bytes = self.cipher.block_size() + data.len();
        output.reserve(needed_bytes);

        unsafe {
            let amnt = crypter.update(data, output.inner_mut()).unwrap();
            output.advance_mut(amnt);
        }
    }

    fn compress_data(&mut self, data: &[u8], output: &mut ByteBuf) {
        let mut coder = ZlibEncoder::new(data, Compression::default());
        output.reserve(coder.total_out() as usize);

        unsafe {
            let amnt = coder.read(output.inner_mut()).unwrap();
            output.advance_mut(amnt);
        }
    }

    fn decompress_data(&mut self, uncompressed_size: i32) {
        let data = &mut self.incoming_compressed;
        if uncompressed_size == 0 {
            self.incoming_uncompressed.reserve(data.len());
            self.incoming_uncompressed.put(data.inner());
        }
        let mut coder = ZlibDecoder::new(data);
        self.incoming_uncompressed
            .reserve(uncompressed_size as usize);
        unsafe {
            let amnt = coder.read(self.incoming_uncompressed.inner_mut()).unwrap();
            self.incoming_uncompressed.advance_mut(amnt);
        }
    }

    pub fn take_pending_packets(&mut self) -> Vec<Box<Packet>> {
        self.pending_received_packets.replace(vec![]).unwrap()
    }
}
