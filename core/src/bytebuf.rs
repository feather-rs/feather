use bytes::{Buf, BufMut};
use std::io::Error;
use std::io::{Read};

#[derive(Clone, Debug)]
pub struct ByteBuf {
    inner: Vec<u8>,
    read_cursor_position: usize,
    write_cursor_position: usize,

    marked_read_position: usize,
    marked_write_position: usize,
}

impl ByteBuf {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            read_cursor_position: 0,
            write_cursor_position: 0,

            marked_read_position: 0,
            marked_write_position: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            inner: vec![],
            read_cursor_position: 0,
            write_cursor_position: 0,

            marked_read_position: 0,
            marked_write_position: 0,
        }
    }

    pub fn read_position(&self) -> usize {
        self.read_cursor_position
    }

    pub fn write_position(&self) -> usize {
        self.write_cursor_position
    }

    pub fn mark_read_position(&mut self) {
        self.marked_read_position = self.read_cursor_position;
    }

    pub fn mark_write_position(&mut self) {
        self.marked_write_position = self.write_cursor_position;
    }

    pub fn reset_read_position(&mut self) {
        self.read_cursor_position = self.marked_read_position;
    }

    pub fn reset_write_position(&mut self) {
        self.write_cursor_position = self.marked_write_position;
    }

    pub fn inner(&self) -> &[u8] {
        &self.inner[self.read_cursor_position..]
    }

    pub fn inner_mut(&mut self) -> &mut [u8] {
        &mut self.inner[self.write_cursor_position..]
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Removes all bytes prior to the current read position.
    pub fn remove_prior(&mut self) {
        let new_capacity = self.inner.capacity() - self.read_cursor_position;
        let mut new_inner = Vec::with_capacity(new_capacity);

        new_inner.extend_from_slice(&self.inner);

        self.inner = new_inner;
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Buf for ByteBuf {
    fn remaining(&self) -> usize {
        self.inner.capacity() - self.read_cursor_position
    }

    fn bytes(&self) -> &[u8] {
        &self.inner[self.read_cursor_position..]
    }

    fn advance(&mut self, cnt: usize) {
        self.read_cursor_position += cnt;
    }
}

impl BufMut for ByteBuf {
    fn remaining_mut(&self) -> usize {
        self.inner.capacity() - self.write_cursor_position
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.write_cursor_position += cnt;
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inner[self.write_cursor_position..]
    }
}

impl Read for ByteBuf {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut amount_read = 0;

        while amount_read < buf.len() {
            let self_index = self.read_cursor_position + amount_read;
            if let Some(val) = self.inner.get(self_index) {
                buf[amount_read] = val.clone();
            } else {
                break;
            }

            amount_read += 1;
        }

        Ok(amount_read)
    }
}
