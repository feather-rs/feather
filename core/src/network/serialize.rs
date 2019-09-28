use crate::network::mctypes::{McTypeRead, McTypeWrite};
use crate::network::packet::{PacketDirection, PacketId, PacketStage};
use crate::{Packet, PacketType};
use aes::Aes128;
use bytes::{Buf, BufMut, BytesMut};
use cfb8::stream_cipher::StreamCipher;
use cfb8::Cfb8;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Cursor, Read, Write};
use tokio::codec::{Decoder, Encoder};
use tokio::io;

type AesCfb8 = Cfb8<Aes128>;

/// Maximum possible size of a varint.
const MAX_VAR_INT_SIZE: usize = 5;
/// Maximum allowed length of a received packet.
const MAX_PACKET_LEN: usize = 1_048_576; // One MB
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "Packet of length {} (under compression threshold {}) was sent compressed",
        _0, _1
    )]
    CompressedPacketTooSmall(usize, usize),
    #[fail(display = "Packet length {} is too large", _0)]
    PacketTooLarge(usize),
    #[fail(display = "Invalid packet ID {} for stage {:?}", _0, _1)]
    InvalidPacketId(u32, PacketStage),
}

/// Codec for encoding and decoding Minecraft packets.
pub struct MinecraftCodec {
    /// Direction of incoming packets.
    incoming_direction: PacketDirection,
    /// The current stage of this codec.
    stage: PacketStage,
    /// The crypter, if encryption is enabled.
    crypter: Option<AesCfb8>,
    /// The compression threshold, if compression is enabled.
    compression_threshold: Option<usize>,
    /// Cached buffer for writing header data.
    /// Using this avoids reallocations.
    header_buffer: BytesMut,
    /// Cached buffer into which we write decompressed
    /// data. Using this avoids reallocations.
    decompressed_buffer: Vec<u8>,
    /// Index into `src` of next byte to decrypt.
    decrypt_index: usize,
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
        let ty = packet.ty();
        trace!("Sending packet with type {:?}", ty);
        dst.push_var_int(ty.get_id().0 as i32);
        packet.write_to(dst);

        // If compression is enabled, we follow a more complex course of action:
        // * Write the raw packet data to `dst`.
        // * If the data is less than the compression threshold, proceed as usual.
        // * Otherwise, we move forward into the buffer, allocating
        // another header and then writing the compressed bytes
        // to the capacity after that.
        let data_len: Option<usize> = if let Some(threshold) = self.compression_threshold {
            let data_len = dst.len();
            if data_len >= threshold {
                // Allocate new header
                dst.reserve(HEADER_SIZE);

                let uncompressed = dst.split_to(data_len);
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

impl Decoder for MinecraftCodec {
    type Item = Box<dyn Packet>;
    type Error = failure::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // If encryption is enabled, decrypt undecrypted data.
        if let Some(crypter) = self.crypter.as_mut() {
            crypter.decrypt(&mut src[self.decrypt_index..]);
            self.decrypt_index = src.len();
        }

        // Conversion to `Cursor` is required because `Bytes` does
        // not implement `Buf`.
        let mut cursor = Cursor::new(src.as_ref());

        // Read header.
        let length = cursor.try_get_var_int()? as usize;

        if length > cursor.remaining() {
            // Full packet has not been read yet.
            return Ok(None);
        }

        // Prevent malicious clients from causing huge allocations.
        if length > MAX_PACKET_LEN {
            return Err(Error::PacketTooLarge(length).into());
        }

        // At this point, we know a full packet has been received.

        // Trim `cursor` and `src` to length of packet.
        let position = cursor.position() as usize;
        src.split_to(position);
        cursor = Cursor::new(&src[..length]);

        // If compression is enabled:
        // * Read the data length field. If 0, continue as normal: the packet is not compressed.
        // * Decompress remaining bytes into `self.decompressed_buffer`.
        // * Update `cursor` to read from `self.decompressed_buffer`.
        if let Some(threshold) = self.compression_threshold {
            let data_length = cursor.try_get_var_int()?;

            if data_length != 0 {
                self.decompressed_buffer.clear();

                let mut decoder = ZlibDecoder::new(cursor);
                decoder.read_to_end(&mut self.decompressed_buffer)?;

                let actual_data_length = self.decompressed_buffer.len();
                if actual_data_length < threshold {
                    return Err(
                        Error::CompressedPacketTooSmall(actual_data_length, threshold).into(),
                    );
                }

                cursor = Cursor::new(&self.decompressed_buffer);
            }
        }

        // Read packet.
        let id = cursor.try_get_var_int()? as u32;
        let packet_type =
            PacketType::get_from_id(PacketId(id, self.incoming_direction, self.stage))
                .map_err(|_| Error::InvalidPacketId(id, self.stage))?;

        let mut packet = packet_type.get_implementation();
        packet.read_from(&mut cursor)?;

        trace!("Received packet with type {:?}", packet_type);

        src.split_to(length);
        self.decrypt_index = 0;

        Ok(Some(packet))
    }
}
