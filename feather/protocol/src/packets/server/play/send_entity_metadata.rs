use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};

use std::fmt;

#[derive(Debug, Clone)]
pub struct SendEntityMetadata {
    entity_id: i32,
    entries: Vec<EntityMetadataEntry>,
}

impl Readable for SendEntityMetadata {
    fn read(buffer: &mut std::io::Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let entity_id = VarInt::read(buffer, version)?.0;
        let mut entries = Vec::new();
        loop {
            let index = u8::read(buffer, version)?;
            if index == 0xff {
                break;
            }
            let entry_type = VarInt::read(buffer, version)?.0;

            match entry_type {
                0 => {
                    let value: u8 = u8::read(buffer, version)?;
                    entries.push(EntityMetadataEntry {
                        index,
                        entry_type: VarInt(entry_type),
                        entry_value: EntityMetadataEntryObject::new_byte(value)
                    });
                }
                1 => {
                    let value: i32 = VarInt::read(buffer, version)?.0;
                }
                2 => {
                    let value: f32 = f32::read(buffer, version)?;
                }
                3 | 4 => {
                    let value: String = String::read(buffer, version)?;
                }
                7 => {
                    let value: bool = bool::read(buffer, version)?;
                }
                _ => {}
            }
        }
        Ok(SendEntityMetadata { entity_id, entries })
    }
}

impl Writeable for SendEntityMetadata {
    fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {}
}

#[derive(Debug, Clone)]
pub struct EntityMetadataEntry {
    pub index: u8,
    pub entry_type: VarInt,
    pub entry_value: EntityMetadataEntryObject,
}

#[derive(Clone)]
pub struct EntityMetadataEntryObject {
    pub byte_value: Option<u8>,
    pub varint_value: Option<VarInt>,
    pub float_value: Option<f32>,
    pub string_value: Option<String>,
    pub bool_value: Option<bool>,
}

impl EntityMetadataEntryObject {
    pub fn new_byte(value: u8) -> Self {
        Self {
            byte_value: Some(value),
            varint_value: None,
            float_value: None,
            string_value: None,
            bool_value: None,
        }
    }
    pub fn new_varint(value: i32) -> Self {
        Self {
            byte_value: None,
            varint_value: Some(VarInt(value)),
            float_value: None,
            string_value: None,
            bool_value: None,
        }
    }
    pub fn new_float(value: f32) -> Self {
        Self {
            byte_value: None,
            varint_value: None,
            float_value: Some(value),
            string_value: None,
            bool_value: None,
        }
    }
    pub fn new_string(value: String) -> Self {
        Self {
            byte_value: None,
            varint_value: None,
            float_value: None,
            string_value: Some(value),
            bool_value: None,
        }
    }
    pub fn new_bool(value: bool) -> Self {
        Self {
            byte_value: None,
            varint_value: None,
            float_value: None,
            string_value: None,
            bool_value: Some(value),
        }
    }
}

impl fmt::Debug for EntityMetadataEntryObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_oject = f.debug_struct("EntityMetadataEntryObject");
        if self.byte_value != None {
            debug_oject.field("byte_value", &self.byte_value);
        } else if self.varint_value != None {
            debug_oject.field("varint_value", &self.varint_value);
        } else if self.float_value != None {
            debug_oject.field("float_value", &self.float_value);
        } else if self.string_value != None {
            debug_oject.field("string_value", &self.string_value);
        } else if self.bool_value != None {
            debug_oject.field("bool_value", &self.bool_value);
        }
        debug_oject.finish()
    }
}