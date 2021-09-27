use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn bool_to_u8<S>(val: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *val {
        false => serializer.serialize_u8(0),
        true => serializer.serialize_u8(1),
    }
}

/// Deserialize a bool from a u8.
/// https://github.com/serde-rs/serde/issues/1344
pub(crate) fn bool_from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        received => Err(D::Error::invalid_value(
            Unexpected::Unsigned(received as u64),
            &"0 or 1",
        )),
    }
}
