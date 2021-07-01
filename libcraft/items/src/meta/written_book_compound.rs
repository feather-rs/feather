use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::{Deserialize, Serialize};

/// Contains data related to written books:
/// * If the book data has been resolved.
/// * The copy tier of the written book.
/// * The title of the written book.
/// * The author of the written book.
/// * The pages of the written book.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrittenBookCompound {
    /// Set to 1 if the book has been opened.
    resolved: Option<u8>,

    /// The copy tier of the written book.
    generation: Option<BookCopyTier>,

    /// The title of the written book.
    title: String,

    /// The author of the written book.
    author: String,

    /// A list of all pages serialized as JSON.
    pages: Vec<String>,
}

/// Enum containing all copy tiers possible in a written
/// book.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPrimitive)]
#[serde(try_from = "u32", into = "u32")]
pub enum BookCopyTier {
    Original = 0,
    CopyOfOriginal = 1,
    CopyOfCopy = 2,
    Tattered = 3,
}

impl Into<u32> for BookCopyTier {
    fn into(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for BookCopyTier {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if let Some(val) = FromPrimitive::from_u32(value) {
            Ok(val)
        } else {
            Err("Cannot find a book copy tier with the provided value!")
        }
    }
}
