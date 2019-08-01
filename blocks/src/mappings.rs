//! Module for reading from block mappings files.
use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use std::collections::HashMap;
use std::io::{Cursor, Read};

pub type NativeBlockIdentifier = (String, Vec<(String, String)>);

#[derive(Debug, Fail)]
pub enum MappingsError {
    #[fail(display = "file did not start with magic string")]
    BadHeader,
    #[fail(display = "invalid boolean value")]
    InvalidBoolean,
    #[fail(display = "mismatched header - wrong native field")]
    MismatchedHeader,
}

#[derive(Clone, Debug)]
pub struct NativeMappings {
    pub version: String,
    pub proto: u32,
    pub blocks: HashMap<NativeBlockIdentifier, u16>,
}

/// Loads a native mappings file.
pub fn load_native(bytes: &[u8]) -> Result<NativeMappings, Error> {
    let mut cursor = Cursor::new(bytes);

    let header = read_header(&mut cursor)?;

    if !header.is_native {
        return Err(MappingsError::MismatchedHeader.into());
    }

    // Read number of mappings
    let len = cursor.read_u32::<LittleEndian>()?;

    let mut blocks = HashMap::with_capacity(len as usize);

    // Read mappings
    for _ in 0..len {
        // Read mapping
        let block_name = cursor.read_string()?;

        // Read properties
        let num_props = cursor.read_u32::<LittleEndian>()?;
        let mut props = vec![];

        for _ in 0..num_props {
            let key = cursor.read_string()?;
            let value = cursor.read_string()?;
            props.push((key, value));
        }

        let identifier = (block_name, props);
        blocks.insert(identifier, cursor.read_u16::<LittleEndian>()?);
    }

    Ok(NativeMappings {
        version: header.version,
        proto: header.proto,
        blocks,
    })
}

#[derive(Clone, Debug)]
pub struct VersionedMappings {
    pub version: String,
    pub proto: u32,
    pub blocks: HashMap<u16, u16>,
}

/// Loads a versioned mappings file.
pub fn load_versioned(bytes: &[u8]) -> Result<VersionedMappings, Error> {
    let mut cursor = Cursor::new(bytes);

    let header = read_header(&mut cursor)?;

    if header.is_native {
        return Err(MappingsError::MismatchedHeader.into());
    }

    // Load blocks
    let len = cursor.read_u32::<LittleEndian>()?;
    let mut blocks = HashMap::new();

    for _ in 0..len {
        let native_id = cursor.read_u16::<LittleEndian>()?;
        let versioned_id = cursor.read_u16::<LittleEndian>()?;
        blocks.insert(native_id, versioned_id);
    }

    Ok(VersionedMappings {
        version: header.version,
        proto: header.proto,
        blocks,
    })
}

#[derive(Clone, Debug)]
struct Header {
    version: String,
    proto: u32,
    is_native: bool,
}

const MAGIC_STRING: &'static str = "FEATHER_BLOCK_DATA_FILE";

fn read_header(cursor: &mut Cursor<&[u8]>) -> Result<Header, Error> {
    let mut buf = vec![0; MAGIC_STRING.len()];
    cursor.read(&mut buf)?;

    let magic = String::from_utf8(buf)?;
    if magic != MAGIC_STRING {
        return Err(MappingsError::BadHeader.into());
    }

    let version = cursor.read_string()?;
    let proto = cursor.read_u32::<LittleEndian>()?;
    let is_native = match cursor.read_u8()? {
        0 => false,
        1 => true,
        _ => return Err(MappingsError::InvalidBoolean.into()),
    };

    Ok(Header {
        version,
        proto,
        is_native,
    })
}

trait ReadExt {
    fn read_string(&mut self) -> Result<String, Error>;
}

impl<R: Read> ReadExt for R {
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_u32::<LittleEndian>()?;
        let mut buf = vec![0; len as usize];

        self.read(&mut buf)?;

        Ok(String::from_utf8(buf)?)
    }
}
