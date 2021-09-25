use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};
use aes::Aes128;
use bytes::BytesMut;
use cfb8::{
    cipher::{AsyncStreamCipher, NewCipher},
    Cfb8,
};
use flate2::{
    bufread::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use std::io::{Cursor, Read};

type AesCfb8 = Cfb8<Aes128>;
pub type CompressionThreshold = usize;

/// An encryption key for use with AES-CFB8.
pub type CryptKey = [u8; 16];

/// State to serialize and deserialize packets from a byte stream.
#[derive(Default)]
pub struct MinecraftCodec {
    /// If encryption is enabled, then this is the cryptor state.
    cryptor: Option<AesCfb8>,
    crypt_key: Option<CryptKey>,
    /// If compression is enabled, then this is the compression threshold.
    compression: Option<CompressionThreshold>,

    /// A buffer of received bytes.
    received_buf: BytesMut,
    /// Auxilary buffer.
    staging_buf: Vec<u8>,
    /// Another auxilary buffer.
    compression_target: Vec<u8>,
}

impl MinecraftCodec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables encryption with the provided key.
    pub fn enable_encryption(&mut self, key: CryptKey) {
        // yes, Mojang uses the same nonce for each packet. don't ask me why.
        self.cryptor = Some(AesCfb8::new_from_slices(&key, &key).expect("key size is invalid"));
        self.crypt_key = Some(key);
    }

    /// Enables compression with the provided compression threshold.
    pub fn enable_compression(&mut self, threshold: CompressionThreshold) {
        self.compression = Some(threshold);
    }

    /// Gets another `MinecraftCodec` with the same compression and encryption
    /// parameters.
    pub fn clone_with_settings(&self) -> MinecraftCodec {
        MinecraftCodec {
            cryptor: self
                .crypt_key
                .map(|key| AesCfb8::new_from_slices(&key, &key).expect("key size is invalid")),
            crypt_key: self.crypt_key,
            compression: self.compression,
            received_buf: BytesMut::new(),
            staging_buf: Vec::new(),
            compression_target: Vec::new(),
        }
    }

    /// Writes a packet into the provided writer.
    pub fn encode(&mut self, packet: &impl Writeable, output: &mut Vec<u8>) -> anyhow::Result<()> {
        packet.write(&mut self.staging_buf, ProtocolVersion::V1_16_2)?;

        if let Some(threshold) = self.compression {
            self.encode_compressed(output, threshold)?;
        } else {
            self.encode_uncompressed(output)?;
        }

        if let Some(cryptor) = &mut self.cryptor {
            cryptor.encrypt(output);
        }

        self.staging_buf.clear();

        Ok(())
    }

    fn encode_compressed(
        &mut self,
        output: &mut Vec<u8>,
        threshold: CompressionThreshold,
    ) -> anyhow::Result<()> {
        let (data_length, data) = if self.staging_buf.len() >= threshold {
            self.data_compressed()
        } else {
            self.data_uncompressed()
        };

        const MAX_VAR_INT_LENGTH: usize = 5;
        let mut buf = [0u8; MAX_VAR_INT_LENGTH];
        let mut data_length_bytes = Cursor::new(&mut buf[..]);
        VarInt(data_length as i32)
            .write_to(&mut data_length_bytes)
            .unwrap();

        let packet_length = data_length_bytes.position() as usize + data.len();
        VarInt(packet_length as i32).write(output, ProtocolVersion::V1_16_2)?;
        VarInt(data_length as i32).write(output, ProtocolVersion::V1_16_2)?;
        output.extend_from_slice(data);

        self.compression_target.clear();

        Ok(())
    }

    fn data_compressed(&mut self) -> (usize, &[u8]) {
        let mut encoder = ZlibEncoder::new(self.staging_buf.as_slice(), Compression::default());
        encoder
            .read_to_end(&mut self.compression_target)
            .expect("compression failed");
        (self.staging_buf.len(), self.compression_target.as_slice())
    }

    fn data_uncompressed(&mut self) -> (usize, &[u8]) {
        (0, self.staging_buf.as_slice())
    }

    fn encode_uncompressed(&mut self, output: &mut Vec<u8>) -> anyhow::Result<()> {
        // TODO: we should probably be able to determine the length without writing the packet,
        // which could remove an unnecessary copy.
        let length = self.staging_buf.len() as i32;
        VarInt(length).write(output, ProtocolVersion::V1_16_2)?;
        output.extend_from_slice(&self.staging_buf);

        Ok(())
    }

    /// Accepts newly received bytes.
    pub fn accept(&mut self, bytes: &[u8]) {
        let start_index = self.received_buf.len();
        self.received_buf.extend(bytes);

        if let Some(cryptor) = &mut self.cryptor {
            // Decrypt the new data (but not the whole received buffer,
            // since old data was already decrypted)
            cryptor.decrypt(&mut self.received_buf[start_index..]);
        }
    }

    /// Gets the next packet that was received, if any.
    pub fn next_packet<T>(&mut self) -> anyhow::Result<Option<T>>
    where
        T: Readable,
    {
        let mut cursor = Cursor::new(&self.received_buf[..]);
        let packet = if let Ok(length) = VarInt::read(&mut cursor, ProtocolVersion::V1_16_2) {
            let length_field_length = cursor.position() as usize;

            if self.received_buf.len() - length_field_length >= length.0 as usize {
                cursor = Cursor::new(
                    &self.received_buf
                        [length_field_length..length_field_length + length.0 as usize],
                );

                if self.compression.is_some() {
                    let data_length = VarInt::read(&mut cursor, ProtocolVersion::V1_16_2)?;
                    if data_length.0 != 0 {
                        let mut decoder =
                            ZlibDecoder::new(&cursor.get_ref()[cursor.position() as usize..]);
                        decoder.read_to_end(&mut self.compression_target)?;
                        cursor = Cursor::new(&self.compression_target);
                    }
                }

                let packet = T::read(&mut cursor, ProtocolVersion::V1_16_2)?;

                let bytes_read = length.0 as usize + length_field_length;
                self.received_buf = self.received_buf.split_off(bytes_read);

                self.compression_target.clear();
                Some(packet)
            } else {
                None
            }
        } else {
            None
        };

        Ok(packet)
    }
}
