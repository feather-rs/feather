use super::world::BlockPosition;
use lemio::ByteBuf;

/// Identifies a type to which Minecraft-specific
/// types (`VarInt`, `VarLong`, etc.) can be written.
pub trait McTypeWrite {
    /// Writes a `VarInt` to the object. See wiki.vg for
    /// details on `VarInt`s and related types.
    fn write_var_int(&mut self, x: i32);
    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn write_string(&mut self, x: &str);

    fn write_position(&mut self, x: &BlockPosition);
}

/// Identifies a type from which Minecraft-specified
/// types can be read.
pub trait McTypeRead {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn read_var_int(&mut self) -> Option<i32>;
    /// Reads a string from the object.
    fn read_string(&mut self) -> Option<String>;

    fn read_position(&mut self) -> Option<BlockPosition>;
}

impl McTypeWrite for ByteBuf {
    /// Writes a `VarInt` to the object. See wiki.vg for
    /// details on `VarInt`s and related types.
    fn write_var_int(&mut self, mut x: i32) {
        loop {
            let mut temp = (x & 0b01111111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b10000000;
            }
            self.write_u8(temp);
            if x == 0 {
                break;
            }
        }
    }
    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn write_string(&mut self, x: &str) {
        let len = x.len();
        self.write_var_int(len as i32);
        let bytes = x.as_bytes();
        bytes.iter().for_each(|b| self.write_u8(b.clone()));
    }

    fn write_position(&mut self, x: &BlockPosition) {
        let result: u64 = ((x.x as u64 & 0x3FFFFFF) << 38)
            | ((x.y as u64 & 0xFFF) << 26)
            | (x.z as u64 & 0x3FFFFFF);
        self.write_u64(result);
    }
}

impl McTypeRead for ByteBuf {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn read_var_int(&mut self) -> Option<i32> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            if self.len() <= self.len() - self.remaining_bytes() {
                // Incomplete VarInt
                return None;
            }
            let read = self.read_u8();
            let value = read & 0b01111111;
            result |= (value as i32) << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return None;
            }
            if read & 0b10000000 == 0 {
                break;
            }
        }
        Some(result)
    }
    /// Reads a string from the object.
    fn read_string(&mut self) -> Option<String> {
        let len = self.read_var_int();
        if let Some(len) = len {
            // Check that the client isn't trying
            // to make the server allocate ridiculous
            // amounts of memory
            if len > 32767 {
                return None;
            }
            let mut result = String::with_capacity(len as usize);
            for _ in 0..len {
                result.push(self.read_u8() as char);
            }
            return Some(result);
        }

        None
    }

    fn read_position(&mut self) -> Option<BlockPosition> {
        let val = self.read_u64();
        let x = val >> 38;
        let y = (val >> 26) & 0xFFF;
        let z = val << 38 >> 38;

        Some(BlockPosition::new(x as u32, y as u32, z as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_var_int() {
        let mut buf = ByteBuf::new();
        buf.write_var_int(0);
        assert_eq!(buf.buffer()[0], 0x00);

        buf = ByteBuf::new();
        buf.write_var_int(128);
        assert_eq!(buf.buffer()[0], 0x80);
        assert_eq!(buf.buffer()[1], 0x01);
    }

    #[test]
    fn read_var_int() {
        let mut buf = ByteBuf::new();
        buf.write_u8(0x00);
        assert_eq!(buf.read_var_int().unwrap(), 0);

        buf = ByteBuf::new();
        buf.write_u8(0x80);
        buf.write_u8(0x01);
        assert_eq!(buf.read_var_int().unwrap(), 128);
    }

    #[test]
    fn write_and_read_var_int() {
        let mut buf = ByteBuf::new();
        buf.write_var_int(893);
        assert_eq!(buf.read_var_int().unwrap(), 893);
    }

    #[test]
    #[should_panic]
    fn read_var_int_too_long() {
        let mut buf = ByteBuf::new();
        buf.write_all(&[
            0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000,
        ]);
        buf.read_var_int().unwrap(); // Should panic
    }

    #[test]
    #[should_panic]
    fn read_var_int_incomplete() {
        let mut buf = ByteBuf::new();
        buf.write_u8(0b10000000);
        buf.read_var_int().unwrap(); // Should panic
    }

    #[test]
    fn write_string() {
        let mut buf = ByteBuf::new();
        buf.write_string("test");
        assert_eq!(buf.buffer().as_slice(), &[0x04, 0x74, 0x65, 0x73, 0x74]);
    }

    #[test]
    fn read_string() {
        let mut buf = ByteBuf::new();
        buf.write_all(&[0x04, 0x74, 0x65, 0x73, 0x74]);
        assert_eq!(buf.read_string().unwrap(), "test");
    }

    #[test]
    #[should_panic]
    fn read_string_too_long() {
        let mut buf = ByteBuf::new();
        buf.write_all(&[
            0xFF, 0xFF, 0xFF, 0xFF, /*random values*/ 0x54, 0x98, 0x84,
        ]);
        buf.read_string().unwrap(); // Should panic
    }

    #[test]
    fn write_and_read_string() {
        let mut buf = ByteBuf::new();
        buf.write_string("test test test {;0?,"); // Put in some random characters for fun
        assert_eq!(buf.read_string().unwrap(), "test test test {;0?,");
    }
}
