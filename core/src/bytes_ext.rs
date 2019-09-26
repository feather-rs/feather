use bytes::{BufMut, Bytes, BytesMut};

/// An error which occurred while attempting
/// to get a value from a `Buf.`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TryGetError {
    /// Indicates that there were not enough remaining
    /// bytes in the buffer to read a value.
    NotEnoughBytes,
    ValueTooLarge,
    InvalidValue,
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
}

macro_rules! try_get_impl {
    ($this:ident, $size:expr, $method:ident) => {{
        if $this.remaining() < $size {
            return Err(TryGetError::NotEnoughBytes);
        }

        return Ok($this.$method());
    }};
}

impl BytesExt for Bytes {
    fn try_get_i8(&mut self) -> Result<i8> {
        try_get_impl!(self, 1, get_i8_be);
    }

    fn try_get_i16(&mut self) -> Result<i8> {
        try_get_impl!(self, 2, get_i16_be);
    }

    fn try_get_i32(&mut self) -> Result<i8> {
        try_get_impl!(self, 4, get_i32_be);
    }

    fn try_get_i64(&mut self) -> Result<i8> {
        try_get_impl!(self, 8, get_i64_be);
    }

    fn try_get_f32(&mut self) -> Result<i8> {
        try_get_impl!(self, 4, get_f32_be);
    }

    fn try_get_f64(&mut self) -> Result<i8> {
        try_get_impl!(self, 8, get_f64_be);
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
}

impl BytesMutExt for BytesMut {
    fn push_i8(&mut self, x: i8) {
        self.reserve(1);
        self.put_i8_be(x);
    }

    fn push_i16(&mut self, x: i16) {
        self.reserve(2);
        self.put_i16_be(x);
    }

    fn push_i32(&mut self, x: i32) {
        self.reserve(4);
        self.put_i32_be(x);
    }

    fn push_i64(&mut self, x: i64) {
        self.reserve(8);
        self.put_i64_be(x);
    }

    fn push_f32(&mut self, x: f32) {
        self.reserve(4);
        self.put_f32_be(x);
    }

    fn push_f64(&mut self, x: f64) {
        self.reserve(8);
        self.put_f64_be(x);
    }
}
