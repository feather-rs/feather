use std::fs;

/// The favicon that appears in the server list on the client.
#[derive(Debug, Clone)]
pub struct Favicon {
    base64_encoded: String,
}

impl Favicon {
    /// Creates a favicon from PNG image data.
    ///
    /// The data is not validated, but malformed
    /// PNGs may cause the client to display an error.
    pub fn from_png(png_bytes: &[u8]) -> Self {
        // See: https://wiki.vg/Server_List_Ping#Response
        let base64 = base64::encode(png_bytes);
        let prefix = "data:image/png;base64,";
        let base64_encoded = format!("{}{}", prefix, base64);
        Self { base64_encoded }
    }

    /// Loads the favicon from its default path
    /// in the current working directory, `server-icon.png`.
    pub fn load_default() -> Option<Self> {
        let path = "server-icon.png";
        let file_contents = fs::read(path).ok()?;
        let favicon = Self::from_png(&file_contents);
        Some(favicon)
    }

    /// Gets base64-encoded PNG data for the `Response` packet.
    pub fn base64_encoded(&self) -> &str {
        &self.base64_encoded
    }
}
