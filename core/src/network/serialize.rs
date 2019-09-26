use crate::network::mctypes::{varint_needed_bytes, McTypeError, McTypeRead, McTypeWrite};
use crate::network::packet::{Packet, PacketDirection, PacketId, PacketStage, PacketType};
use aes::Aes128;
use bytes::{Buf, BytesMut};
use cfb8::stream_cipher::StreamCipher;
use cfb8::Cfb8;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use std::io::{Cursor, Write};
use tokio::codec::{Decoder, Encoder};
use tokio::io;

type AesCfb8 = Cfb8<Aes128>;

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

/// The maxmimum size of a packet header.
///
/// This size does not account for the compressed packet format
/// or the packet ID field.
const HEADER_SIZE: usize = MAX_VAR_INT_SIZE * 2; // One VarInt for length and one for uncompressed length (for compressed format)

/// Codec for encoding and decoding Minecraft packets.
#[derive(Debug)]
pub struct MinecraftCodec {
    /// The current stage of this codec.
    stage: PacketStage,
    /// The crypter, if encryption is enabled.
    crypter: Option<AesCfb8>,
    /// The compression threshold, if compression is enabled.
    compression_threshold: Option<usize>,

    /// The index of the next byte in the buffer to decrypt.
    read_index: usize,
}

impl Encoder for MinecraftCodec {
    type Item = Box<dyn Packet>;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, mut dst: &mut BytesMut) -> Result<(), Self::Error> {
        trace!("Sending packet with type {:?}", packet.ty());
        dst.reserve(HEADER_SIZE + MAX_VAR_INT_SIZE + packet.needed_bytes()); // Additional MAX_VAR_INT_SIZE is for packet ID

        let packet_id = packet.ty().get_id().0 as i32;

        // Split the buffer so that the header can be written later.
        // `dst` now contains the header.
        let mut packet_data = dst.split_off(HEADER_SIZE);
        packet_data.put_var_int(packet_id);
        packet.write_to(&mut packet_data);

        let packet_len = packet_data.len() as i32;

        if let Some(threshold) = self.compression_threshold.as_ref() {
            // Compression is enabled - compress if needed.

            if packet_len >= *threshold as i32 {
                // Instead of allocating a new buffer for compression,
                // we just reserve some additional space in the destination buffer
                // and skip the uncompressed bytes. This allows for less
                // allocations, because the allocated space will later be reused
                // by BytesMut::reserve() rather than discarded.

                // Reserve a new header, since we're skipping the old one.
                // Also reserve space for the compressed packet data, which we'll
                // assume to be the same size as the uncompressed data to avoid
                // another allocation.
                packet_data.reserve(HEADER_SIZE + packet_len as usize);

                // Packet is large enough to be compressed - compress it.
                let uncompressed_len = packet_len;

                // `header` will contain the bytes reserved above, while
                // `packet_data` will still contain the uncompressed packet
                // data.
                let mut header = packet_data.split_off(packet_len as usize);

                // `compressed_data` will be a buffer with capacity `packet_len`
                // contiguous with the header. This allows unsplitting
                // the header and the data, efficiently merging the two buffers.
                let mut compressed_data = header.split_off(HEADER_SIZE);

                // Compress uncompressed data in `packet_buf` and write it to `compressed_data`.
                let mut encoder = ZlibEncoder::new(compressed_data, Compression::default());
                encoder.write_all(&packet_data.bytes())?;
                let mut compressed_data = encoder.finish()?;

                *dst = write_compressed_header(uncompressed_len as usize, header, compressed_data);
            } else {
                // Don't compress the data, since its length is shorter than the compression threshold.
                // The uncompressed length field in the header will be set to 0.
                let mut header = dst.split_off(0);

                *dst = write_compressed_header(0, header, packet_data);
            }
        } else {
            // Compression isn't enabled - write header as usual.
            let header_size = varint_needed_bytes(packet_len as i32);

            let mut header = dst.split_off(HEADER_SIZE - header_size);

            header.put_var_int(packet_len as i32);

            header.unsplit(packet_data);
            *dst = header;
        }

        // If encryption is enabled, encrypt the data in place.
        if let Some(crypter) = self.crypter.as_mut() {
            crypter.encrypt(&mut dst[..]);
        }

        Ok(())
    }
}

impl Decoder for MinecraftCodec {
    type Item = Box<dyn Packet>;
    type Error = failure::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Decrypt new data if encryption is enabled.
        if let Some(crypter) = self.crypter.as_mut() {
            crypter.decrypt(&mut src[self.read_index..]);
            self.read_index = src.len();
        }

        // If the packet wasn't fully read, we don't
        // want `src`'s read position to be advanced.
        // Use this temporary cursor until we know
        // the whole packet is read.
        let mut cursor = Cursor::new(src.bytes());

        // Attempt to read a VarInt (the packet length).
        // If there aren't enough bytes to read the length,
        // return, since the entire packet hasn't arrived
        // yet. If another error occurred, the VarInt was corrupt,
        // so return an error.
        let len = match cursor.try_get_var_int() {
            Ok(len) => len as usize,
            Err(e) => match e {
                McTypeError::NotEnoughBytes(_) => return Ok(None),
                e => return Err(e.into()),
            },
        };

        // Check that the entire packet is in memory.
        if cursor.remaining() < len {
            return Err(None);
        }

        // The whole packet is in `src`, so advance
        // `src` by the number of bytes already read.
        src.advance(cursor.position() as usize);

        // Remove the bytes from `src` so future calls
        // to decode() won't read the old bytes.
        let mut src = src.split_to(len).freeze();

        // If compression is enabled, decompress the packet if needed.
        if let Some(threshold) = self.compression_threshold.as_ref() {
            let uncompressed_length = src.try_get_var_int()? as usize;
            if uncompressed_length != 0 {
                // Allocate a new buffer and decompress data into buffer.

                if uncompressed_length > 1_048_576 {
                    return Err(Error::PacketLengthTooLarge(uncompressed_length).into());
                }

                if uncompressed_length < *threshold {
                    return Err(
                        Error::CompressedPacketTooSmall(uncompressed_length, *threshold).into(),
                    );
                }

                let mut buf = BytesMut::with_capacity(uncompressed_length);

                let mut decoder = ZlibDecoder::new(buf);
                decoder.write_all(&src.bytes())?;
                src = decoder.finish()?.freeze();
            }
        }

        let packet_id = PacketId(
            src.try_get_var_int()? as u32,
            PacketDirection::Serverbound,
            self.stage,
        );
        let packet_ty = match PacketType::get_from_id(packet_id) {
            Ok(ty) => ty,
            Err(_) => return Err(Error::InvalidPacketId(packet_id.0, self.stage).into()),
        };

        trace!("Received packet with type {:?}", packet_ty);

        let mut packet = packet_ty.get_implementation();
        packet.read_from(&mut src);

        Ok(packet)
    }
}

/// Writes the packet header using the compressed packet format
/// and then unsplits the header from the data.
fn write_compressed_header(
    uncompressed_len: usize,
    mut header: BytesMut,
    compressed_data: BytesMut,
) -> BytesMut {
    let full_packet_len = varint_needed_bytes(uncompressed_len as i32) + compressed_data.len();

    let header_size =
        varint_needed_bytes(full_packet_len as i32) + varint_needed_bytes(uncompressed_len as i32);

    // Drop unneeded bytes in the header. Since its capacity is the
    // maximum length of the header, and the header has a variable
    // length, there's no need to write to unused bytes.
    let mut header = header.split_off(HEADER_SIZE - header_size);

    header.put_var_int(full_packet_len as i32);
    header.put_var_int(uncompressed_len as i32);

    header.unsplit(compressed_data);

    header
}
