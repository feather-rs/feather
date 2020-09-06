use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};
use aes::Aes128;
use bytes::BytesMut;
use cfb8::{
    stream_cipher::{NewStreamCipher, StreamCipher},
    Cfb8,
};
use std::io::Cursor;

type AesCfb8 = Cfb8<Aes128>;
pub type CompressionThreshold = usize;

/// An encryption key for use with AES-CFB8.
pub type CryptKey = [u8; 16];

/// State to serialize and deserialize packets from a byte stream.
#[derive(Default)]
pub struct MinecraftCodec {
    /// If encryption is enabled, then this is the cryptor state.
    cryptor: Option<AesCfb8>,
    /// If compression is enabled, then this is the compression threshold.
    compression: Option<CompressionThreshold>,

    /// A buffer of received bytes.
    received_buf: BytesMut,
    /// Auxilary buffer for use with compression.
    staging_buf: Vec<u8>,
}

impl MinecraftCodec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables encryption with the provided key.
    pub fn enable_encryption(&mut self, key: CryptKey) {
        // yes, Mojang uses the same nonce for each packet. don't ask me why.
        self.cryptor = Some(AesCfb8::new_var(&key, &key).expect("key size is invalid"));
    }

    /// Enables compression with the provided compression threshold.
    pub fn enable_compression(&mut self, threshold: CompressionThreshold) {
        self.compression = Some(threshold);
    }

    /// Writes a packet into the provided writer.
    pub fn encode(&mut self, packet: &impl Writeable, output: &mut Vec<u8>) {
        packet.write(&mut self.staging_buf, ProtocolVersion::V1_16_2);
        if let Some(threshold) = self.compression {
            self.encode_compressed(output, threshold);
        } else {
            self.encode_uncompressed(output);
        }

        if let Some(cryptor) = &mut self.cryptor {
            cryptor.encrypt(output);
        }

        self.staging_buf.clear();
    }

    fn encode_compressed(&mut self, output: &mut Vec<u8>, threshold: CompressionThreshold) {
        todo!()
    }

    fn encode_uncompressed(&mut self, output: &mut Vec<u8>) {
        // TODO: we should probably be able to determine the length without writing the packet,
        // which could remove an unnecessary copy.
        let length = self.staging_buf.len() as i32;
        VarInt(length).write(output, ProtocolVersion::V1_16_2);
        output.extend_from_slice(&self.staging_buf);
    }

    /// Receives some bytes. Calls `received_packet` each time
    /// a packet is decoded.
    pub fn decode<T>(
        &mut self,
        bytes: &[u8],
        mut received_packet: impl FnMut(T),
    ) -> anyhow::Result<()>
    where
        T: Readable,
    {
        let start_index = self.received_buf.len();
        self.received_buf.extend_from_slice(bytes);

        if let Some(cryptor) = &mut self.cryptor {
            // Decrypt the new data (but not the whole received buffer,
            // since old data was already decrypted)
            cryptor.decrypt(&mut self.received_buf[start_index..]);
        }

        let mut cursor = Cursor::new(&self.received_buf[..]);
        while let Ok(length) = VarInt::read(&mut cursor, ProtocolVersion::V1_16_2) {
            if self.received_buf.len() - cursor.position() as usize >= length.0 as usize {
                let packet = T::read(&mut cursor, ProtocolVersion::V1_16_2)?;
                received_packet(packet);
            } else {
                break;
            }
        }
        self.received_buf = self.received_buf.split_off(cursor.position() as usize);

        Ok(())
    }
}
