use crate::network::mctypes::McTypeWrite;
use crate::network::packet::PacketStage;
use crate::Packet;
use aes::Aes128;
use bytes::{BufMut, BytesMut};
use cfb8::stream_cipher::StreamCipher;
use cfb8::Cfb8;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use tokio::codec::Encoder;
use tokio::io;

type AesCfb8 = Cfb8<Aes128>;

const MAX_VAR_INT_SIZE: usize = 5;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "Packet of length {} (under compression threshold {}) was sent compressed",
        _0, _1
    )]
    CompressedPacketTooSmall(usize, usize),
    #[fail(display = "Packet length {} is too large", _0)]
    PacketLengthTooLarge(usize),
    #[fail(display = "Invalid packet ID {} for stage {:?}", _0, _1)]
    InvalidPacketId(u32, PacketStage),
}

/// Codec for encoding and decoding Minecraft packets.
#[derive()]
pub struct MinecraftCodec {
    /// The current stage of this codec.
    stage: PacketStage,
    /// The crypter, if encryption is enabled.
    crypter: Option<AesCfb8>,
    /// The compression threshold, if compression is enabled.
    compression_threshold: Option<usize>,

    /// The index of the next byte in the buffer to decrypt.
    read_index: usize,

    /// Cached buffer for writing header data.
    /// This avoids reallocations.
    header_buffer: BytesMut,
}

impl Encoder for MinecraftCodec {
    type Item = Box<dyn Packet>;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Reserve space for the packet header (at most 2 * 5 bytes, for length + data length).
        // `header` will contain the first 10 bytes of the buffer, while `dst`
        // still contains the rest.
        // "Data length" refers to the uncompressed size of the packet.
        // Since we cannot know the size of the header in advance, thanks to varints,
        // we reserve the maximum size and copy the header in with a correct offset.
        const HEADER_SIZE: usize = MAX_VAR_INT_SIZE * 2;
        dst.reserve(HEADER_SIZE);
        let mut header = dst.split_to(HEADER_SIZE);
        assert!(dst.is_empty());
        assert!(header.is_empty());

        // Write raw packet data to `dst`.
        trace!("Sending packet with type {:?}", packet.ty());
        dst.push_var_int(packet.ty().get_id().0 as i32);
        packet.write_to(dst);

        // If compression is enabled, we follow a more complex course of action:
        // * Write the raw packet data to `dst`.
        // * If the data is less than the compression threshold, proceed as usual.
        // * Otherwise, we move forward into the buffer, allocating
        // another header and then writing the compressed bytes
        // to the capacity after that.
        let mut data_len: Option<usize> = if let Some(threshold) = self.compression_threshold {
            let data_len = dst.len();
            if data_len >= threshold {
                // Allocate new header
                dst.reserve(HEADER_SIZE);

                let mut uncompressed = dst.split_to(data_len);
                header = dst.split_to(HEADER_SIZE);

                assert!(dst.is_empty());
                // Compress data into `compressed`.
                let mut encoder = ZlibEncoder::new(dst.writer(), Compression::default());
                encoder.write_all(uncompressed.as_ref()).unwrap();

                Some(data_len)
            } else {
                Some(0) // Not compressed
            }
        } else {
            None
        };

        // If encryption is enabled, encrypt data in place.
        if let Some(crypter) = self.crypter.as_mut() {
            crypter.encrypt(dst);
        }

        // Write header. We first write to a temporary buffer,
        // then copy this to the correct position in `header`,
        // trim off the unused bytes.
        let mut used_header_bytes = 0;
        used_header_bytes += self.header_buffer.push_var_int(dst.len() as i32);
        if let Some(data_len) = data_len {
            used_header_bytes += self.header_buffer.push_var_int(data_len as i32);
        }

        // Offset into `header` to write to.
        let header_offset = HEADER_SIZE - used_header_bytes;
        // Discard unused header bytes.
        header.split_to(header_offset);

        // Write into header.
        header.extend_from_slice(&self.header_buffer);
        self.header_buffer.clear();

        // Finally, merge `header` and `dst`.
        std::mem::swap(dst, &mut header);
        dst.unsplit(header);

        Ok(())
    }
}
