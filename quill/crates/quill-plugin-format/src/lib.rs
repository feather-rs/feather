//! Defines the file format used for compiled [`quill`](https://github.com/feather-rs/quill)
//! plugins.
//!
//! Currently, the file format is based on gzipped `tar` files.
mod metadata;

use std::{
    borrow::Cow,
    io::{Cursor, Read, Write},
};

use anyhow::anyhow;
use flate2::Compression;
use tar::Header;

pub use metadata::PluginMetadata;

const METADATA_PATH: &str = "metadata.json";
const WASM_BYTECODE_PATH: &str = "module.wasm";

/// A plugin definition stored in the Quill file format.
pub struct PluginFile<'a> {
    wasm_bytecode: Cow<'a, [u8]>,
    metadata: PluginMetadata,
}

impl<'a> PluginFile<'a> {
    pub fn new(wasm_bytecode: impl Into<Cow<'a, [u8]>>, metadata: PluginMetadata) -> Self {
        Self {
            wasm_bytecode: wasm_bytecode.into(),
            metadata,
        }
    }

    pub fn wasm_bytecode(&self) -> &[u8] {
        &*self.wasm_bytecode
    }

    pub fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    /// Writes this plugin into a `Vec`.
    ///
    /// `compression_level` should be between 0 (worst) and 9 (best, but slow).
    pub fn encode(&self, compression_level: u32) -> Vec<u8> {
        let vec = Vec::new();
        let mut archive_builder = tar::Builder::new(flate2::write::GzEncoder::new(
            vec,
            Compression::new(compression_level),
        ));

        self.write_metadata(&mut archive_builder);
        self.write_wasm_bytecode(&mut archive_builder);

        archive_builder
            .into_inner()
            .expect("write to Vec failed")
            .finish()
            .expect("compression failed")
    }

    fn write_metadata(&self, archive_builder: &mut tar::Builder<impl Write>) {
        let metadata = serde_json::to_string_pretty(&self.metadata)
            .expect("failed to serialize PluginMetadata");

        let mut header = Header::new_gnu();
        header.set_size(metadata.len() as u64);
        header.set_mode(0o644);

        archive_builder
            .append_data(
                &mut header,
                METADATA_PATH,
                Cursor::new(metadata.into_bytes()),
            )
            .expect("write to Vec failed");
    }

    fn write_wasm_bytecode(&self, archive_builder: &mut tar::Builder<impl Write>) {
        let mut header = Header::new_gnu();
        header.set_size(self.wasm_bytecode.len() as u64);
        header.set_mode(0o644);

        archive_builder
            .append_data(
                &mut header,
                WASM_BYTECODE_PATH,
                Cursor::new(self.wasm_bytecode()),
            )
            .expect("write to Vec failed");
    }
}

impl PluginFile<'static> {
    /// Deserializes a plugin file.
    pub fn decode(data: impl Read) -> anyhow::Result<Self> {
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(data));

        let mut metadata = None;
        let mut wasm_bytecode = None;
        for entry in archive.entries()? {
            let entry = entry?;

            match &*entry.path()?.to_string_lossy() {
                s if s == METADATA_PATH => metadata = Some(Self::decode_metadata(entry)?),
                s if s == WASM_BYTECODE_PATH => {
                    wasm_bytecode = Some(Self::decode_wasm_bytecode(entry)?)
                }
                _ => (),
            }
        }

        let metadata =
            metadata.ok_or_else(|| anyhow!("missing plugin metadata ({})", METADATA_PATH))?;
        let wasm_bytecode = wasm_bytecode
            .ok_or_else(|| anyhow!("missing WASM bytecode ({})", WASM_BYTECODE_PATH))?
            .into();
        Ok(Self {
            metadata,
            wasm_bytecode,
        })
    }

    fn decode_metadata(reader: impl Read) -> anyhow::Result<PluginMetadata> {
        serde_json::from_reader(reader).map_err(anyhow::Error::from)
    }

    fn decode_wasm_bytecode(mut reader: impl Read) -> anyhow::Result<Vec<u8>> {
        let mut wasm = Vec::new();
        reader.read_to_end(&mut wasm)?;
        Ok(wasm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let wasm_bytecode = vec![0xFF; 1024];
        let metadata = PluginMetadata {
            name: "TestPlugin".to_owned(),
            identifier: "test-plugin".to_owned(),
            version: "0.1.0".to_owned(),
            api_version: "0.1.0".to_owned(),
            description: Some("test plugin".to_owned()),
            authors: vec!["caelunshun".to_owned()],
        };
        let file = PluginFile::new(wasm_bytecode.clone(), metadata.clone());
        let encoded = file.encode(8);
        let decoded = PluginFile::decode(Cursor::new(encoded)).unwrap();

        assert_eq!(decoded.metadata(), &metadata);
        assert_eq!(decoded.wasm_bytecode(), wasm_bytecode);
    }
}
