use serde::{Deserialize, Serialize};

/// Contains related data to books and quills:
/// * The pages of the book and quill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookAndQuillCompound {
    /// A list of all pages serialized as JSON.
    pages: Vec<String>,
}
