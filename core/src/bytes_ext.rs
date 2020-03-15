use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;

/// An error which occurred while attempting
/// to get a value from a `Buf.`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum TryGetError {
    /// Indicates that there were not enough remaining
    /// bytes in the buffer to read a value.
    #[error("not enough bytes left in buffer")]
    NotEnoughBytes,
    #[error("value too large")]
    ValueTooLarge,
    #[error("invalid value {0}")]
    InvalidValue(i32),
}

type Result<T> = std::result::Result<T, TryGetError>;

/// Ext trait for `Bytes` to allow `try_get` operations.
///
/// All operations are big-endian.
pub trait BytesExt {
    fn try_get_i8(&mut self) -> Result<i8>;
    fn try_get_i16(&mut self) -> Result<i16>;
    fn try_get_i32(&mut self) -> Result<i32>;
    fn try_get_i64(&mut self) -> Result<i64>;

    fn try_get_f32(&mut self) -> Result<f32>;
    fn try_get_f64(&mut self) -> Result<f64>;

    fn try_get_u8(&mut self) -> Result<u8>;
    fn try_get_u16(&mut self) -> Result<u16>;
    fn try_get_u32(&mut self) -> Result<u32>;
    fn try_get_u64(&mut self) -> Result<u64>;
}

macro_rules! try_get_impl {
    ($this:ident, $size:expr, $method:ident) => {{
        if $this.remaining() < $size {
            return Err(TryGetError::NotEnoughBytes);
        }

        return Ok($this.$method());
    }};
}

impl<B: Buf> BytesExt for B {
    fn try_get_i8(&mut self) -> Result<i8> {
        try_get_impl!(self, 1, get_i8);
    }

    fn try_get_i16(&mut self) -> Result<i16> {
        try_get_impl!(self, 2, get_i16);
    }

    fn try_get_i32(&mut self) -> Result<i32> {
        try_get_impl!(self, 4, get_i32);
    }

    fn try_get_i64(&mut self) -> Result<i64> {
        try_get_impl!(self, 8, get_i64);
    }

    fn try_get_f32(&mut self) -> Result<f32> {
        try_get_impl!(self, 4, get_f32);
    }

    fn try_get_f64(&mut self) -> Result<f64> {
        try_get_impl!(self, 8, get_f64);
    }

    fn try_get_u8(&mut self) -> Result<u8> {
        try_get_impl!(self, 1, get_u8);
    }

    fn try_get_u16(&mut self) -> Result<u16> {
        try_get_impl!(self, 2, get_u16);
    }

    fn try_get_u32(&mut self) -> Result<u32> {
        try_get_impl!(self, 4, get_u32);
    }

    fn try_get_u64(&mut self) -> Result<u64> {
        try_get_impl!(self, 8, get_u64);
    }
}

/// Ext trait to implement put operations
/// for `BytesMut` which reserve additional capacity
/// rather than panicking.
pub trait BytesMutExt {
    fn push_i8(&mut self, x: i8);
    fn push_i16(&mut self, x: i16);
    fn push_i32(&mut self, x: i32);
    fn push_i64(&mut self, x: i64);

    fn push_f32(&mut self, x: f32);
    fn push_f64(&mut self, x: f64);

    fn push_u8(&mut self, x: u8);
    fn push_u16(&mut self, x: u16);
    fn push_u32(&mut self, x: u32);
    fn push_u64(&mut self, x: u64);
}

impl BytesMutExt for BytesMut {
    fn push_i8(&mut self, x: i8) {
        self.reserve(1);
        self.put_i8(x);
    }

    fn push_i16(&mut self, x: i16) {
        self.reserve(2);
        self.put_i16(x);
    }

    fn push_i32(&mut self, x: i32) {
        self.reserve(4);
        self.put_i32(x);
    }

    fn push_i64(&mut self, x: i64) {
        self.reserve(8);
        self.put_i64(x);
    }

    fn push_f32(&mut self, x: f32) {
        self.reserve(4);
        self.put_f32(x);
    }

    fn push_f64(&mut self, x: f64) {
        self.reserve(8);
        self.put_f64(x);
    }

    fn push_u8(&mut self, x: u8) {
        self.reserve(1);
        self.put_u8(x);
    }

    fn push_u16(&mut self, x: u16) {
        self.reserve(2);
        self.put_u16(x);
    }

    fn push_u32(&mut self, x: u32) {
        self.reserve(4);
        self.put_u32(x);
    }

    fn push_u64(&mut self, x: u64) {
        self.reserve(8);
        self.put_u64(x);
    }
}
