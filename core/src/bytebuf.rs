use bytes::{Buf, BufMut};
use std::io::Error;
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct ByteBuf {
    inner: Vec<u8>,
    read_cursor_position: usize,

    marked_read_position: usize,
}

impl ByteBuf {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            read_cursor_position: 0,

            marked_read_position: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            inner: vec![],
            read_cursor_position: 0,

            marked_read_position: 0,
        }
    }

    pub fn read_pos(&self) -> usize {
        self.read_cursor_position
    }

    pub fn mark_read_position(&mut self) {
        self.marked_read_position = self.read_cursor_position;
    }

    pub fn marked_read_position(&self) -> usize {
        self.marked_read_position
    }

    pub fn reset_read_position(&mut self) {
        self.read_cursor_position = self.marked_read_position;
    }

    pub fn inner(&self) -> &[u8] {
        &self.inner[self.read_cursor_position..]
    }

    pub unsafe fn inner_mut(&mut self) -> &mut [u8] {
        &mut std::slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.inner.capacity())
            [self.inner.len()..]
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Removes all bytes prior to the current read position.
    pub fn remove_prior(&mut self) {
        let new_capacity = self.inner.capacity() - self.read_cursor_position;
        let mut new_inner = Vec::with_capacity(new_capacity);

        new_inner.extend_from_slice(&self.inner[self.read_cursor_position..]);
        self.read_cursor_position = 0;

        self.inner = new_inner;
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}

impl Default for ByteBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl Buf for ByteBuf {
    fn remaining(&self) -> usize {
        self.inner.len() - self.read_cursor_position
    }

    fn bytes(&self) -> &[u8] {
        unsafe {
            &std::slice::from_raw_parts(self.inner.as_ptr(), self.inner.capacity())
                [self.read_cursor_position..]
        }
    }

    fn advance(&mut self, cnt: usize) {
        self.read_cursor_position += cnt;
    }
}

impl BufMut for ByteBuf {
    fn remaining_mut(&self) -> usize {
        self.inner.capacity() - self.inner.len()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.inner.set_len(self.inner.len() + cnt);
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        &mut std::slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.inner.capacity())
            [self.inner.len()..]
    }
}

impl Read for ByteBuf {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut amount_read = 0;

        while amount_read < buf.len() {
            let self_index = self.read_cursor_position + amount_read;
            if let Some(val) = self.inner.get(self_index) {
                buf[amount_read] = *val;
            } else {
                break;
            }

            amount_read += 1;
        }

        Ok(amount_read)
    }
}

impl Write for ByteBuf {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.reserve(buf.len());
        buf.iter().for_each(|x| self.write_u8(*x));
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// Trait which contains functions like those
/// in `BufMut` but which allocate additional
/// space rather than panicking when a buffer is full.
pub trait BufMutAlloc {
    fn write_f32_be(&mut self, x: f32);
    fn write_f32_le(&mut self, x: f32);
    fn write_f64_be(&mut self, x: f64);
    fn write_f64_le(&mut self, x: f64);

    fn write_u8(&mut self, x: u8);
    fn write_u16_be(&mut self, x: u16);
    fn write_u16_le(&mut self, x: u16);
    fn write_u32_be(&mut self, x: u32);
    fn write_u32_le(&mut self, x: u32);
    fn write_u64_be(&mut self, x: u64);
    fn write_u64_le(&mut self, x: u64);

    fn write_i8(&mut self, x: i8);
    fn write_i16_be(&mut self, x: i16);
    fn write_i16_le(&mut self, x: i16);
    fn write_i32_be(&mut self, x: i32);
    fn write_i32_le(&mut self, x: i32);
    fn write_i64_be(&mut self, x: i64);
    fn write_i64_le(&mut self, x: i64);
}

impl BufMutAlloc for ByteBuf {
    fn write_f32_be(&mut self, x: f32) {
        self.reserve(4);
        self.put_f32_be(x);
    }

    fn write_f32_le(&mut self, x: f32) {
        self.reserve(4);
        self.put_f32_le(x);
    }

    fn write_f64_be(&mut self, x: f64) {
        self.reserve(8);
        self.put_f64_be(x);
    }

    fn write_f64_le(&mut self, x: f64) {
        self.reserve(8);
        self.put_f64_le(x);
    }

    fn write_u8(&mut self, x: u8) {
        self.reserve(1);
        self.put_u8(x);
    }

    fn write_u16_be(&mut self, x: u16) {
        self.reserve(2);
        self.put_u16_be(x);
    }

    fn write_u16_le(&mut self, x: u16) {
        self.reserve(2);
        self.put_u16_le(x);
    }

    fn write_u32_be(&mut self, x: u32) {
        self.reserve(4);
        self.put_u32_be(x);
    }

    fn write_u32_le(&mut self, x: u32) {
        self.reserve(4);
        self.put_u32_le(x);
    }

    fn write_u64_be(&mut self, x: u64) {
        self.reserve(8);
        self.put_u64_be(x);
    }

    fn write_u64_le(&mut self, x: u64) {
        self.reserve(8);
        self.put_u64_le(x);
    }

    fn write_i8(&mut self, x: i8) {
        self.reserve(1);
        self.put_i8(x);
    }

    fn write_i16_be(&mut self, x: i16) {
        self.reserve(2);
        self.put_i16_be(x);
    }

    fn write_i16_le(&mut self, x: i16) {
        self.reserve(2);
        self.put_i16_le(x);
    }

    fn write_i32_be(&mut self, x: i32) {
        self.reserve(4);
        self.put_i32_be(x);
    }

    fn write_i32_le(&mut self, x: i32) {
        self.reserve(4);
        self.put_i32_le(x);
    }

    fn write_i64_be(&mut self, x: i64) {
        self.reserve(8);
        self.put_i64_be(x);
    }

    fn write_i64_le(&mut self, x: i64) {
        self.reserve(8);
        self.put_i64_le(x);
    }
}

pub trait BufResulted {
    fn byte_check(&self, amnt: usize) -> Result<(), ()>;

    fn read_f32_be(&mut self) -> Result<f32, ()>;
    fn read_f32_le(&mut self) -> Result<f32, ()>;
    fn read_f64_be(&mut self) -> Result<f64, ()>;
    fn read_f64_le(&mut self) -> Result<f64, ()>;

    fn read_u8(&mut self) -> Result<u8, ()>;
    fn read_u16_be(&mut self) -> Result<u16, ()>;
    fn read_u16_le(&mut self) -> Result<u16, ()>;
    fn read_u32_be(&mut self) -> Result<u32, ()>;
    fn read_u32_le(&mut self) -> Result<u32, ()>;
    fn read_u64_be(&mut self) -> Result<u64, ()>;
    fn read_u64_le(&mut self) -> Result<u64, ()>;

    fn read_i8(&mut self) -> Result<i8, ()>;
    fn read_i16_be(&mut self) -> Result<i16, ()>;
    fn read_i16_le(&mut self) -> Result<i16, ()>;
    fn read_i32_be(&mut self) -> Result<i32, ()>;
    fn read_i32_le(&mut self) -> Result<i32, ()>;
    fn read_i64_be(&mut self) -> Result<i64, ()>;
    fn read_i64_le(&mut self) -> Result<i64, ()>;
}

impl<T: Buf> BufResulted for T {
    #[inline]
    fn byte_check(&self, amnt: usize) -> Result<(), ()> {
        if self.remaining() >= amnt {
            Ok(())
        } else {
            Err(())
        }
    }

    fn read_f32_be(&mut self) -> Result<f32, ()> {
        self.byte_check(4)?;
        Ok(self.get_f32_be())
    }

    fn read_f32_le(&mut self) -> Result<f32, ()> {
        self.byte_check(4)?;
        Ok(self.get_f32_le())
    }

    fn read_f64_be(&mut self) -> Result<f64, ()> {
        self.byte_check(8)?;
        Ok(self.get_f64_be())
    }

    fn read_f64_le(&mut self) -> Result<f64, ()> {
        self.byte_check(8)?;
        Ok(self.get_f64_le())
    }

    fn read_u8(&mut self) -> Result<u8, ()> {
        self.byte_check(1)?;
        Ok(self.get_u8())
    }

    fn read_u16_be(&mut self) -> Result<u16, ()> {
        self.byte_check(2)?;
        Ok(self.get_u16_be())
    }

    fn read_u16_le(&mut self) -> Result<u16, ()> {
        self.byte_check(2)?;
        Ok(self.get_u16_le())
    }

    fn read_u32_be(&mut self) -> Result<u32, ()> {
        self.byte_check(4)?;
        Ok(self.get_u32_be())
    }

    fn read_u32_le(&mut self) -> Result<u32, ()> {
        self.byte_check(4)?;
        Ok(self.get_u32_le())
    }

    fn read_u64_be(&mut self) -> Result<u64, ()> {
        self.byte_check(8)?;
        Ok(self.get_u64_be())
    }

    fn read_u64_le(&mut self) -> Result<u64, ()> {
        self.byte_check(8)?;
        Ok(self.get_u64_le())
    }

    fn read_i8(&mut self) -> Result<i8, ()> {
        self.byte_check(1)?;
        Ok(self.get_i8())
    }

    fn read_i16_be(&mut self) -> Result<i16, ()> {
        self.byte_check(2)?;
        Ok(self.get_i16_be())
    }

    fn read_i16_le(&mut self) -> Result<i16, ()> {
        self.byte_check(2)?;
        Ok(self.get_i16_le())
    }

    fn read_i32_be(&mut self) -> Result<i32, ()> {
        self.byte_check(4)?;
        Ok(self.get_i32_be())
    }

    fn read_i32_le(&mut self) -> Result<i32, ()> {
        self.byte_check(4)?;
        Ok(self.get_i32_le())
    }

    fn read_i64_be(&mut self) -> Result<i64, ()> {
        self.byte_check(8)?;
        Ok(self.get_i64_be())
    }

    fn read_i64_le(&mut self) -> Result<i64, ()> {
        self.byte_check(8)?;
        Ok(self.get_i64_le())
    }
}
