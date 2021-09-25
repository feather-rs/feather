//! Defines the file format used for compiled [`quill`](https://github.com/feather-rs/quill)
//! plugins.
//!
//! Currently, the file format is based on gzipped `tar` files.
mod metadata;

use std::{
    borrow::Cow,
    io::{Cursor, Read, Write},
};

use anyhow::{anyhow, bail};
use flate2::Compression;
use tar::Header;

pub use metadata::{PluginMetadata, PluginTarget};

use target_lexicon::OperatingSystem;
pub use target_lexicon::Triple;

const METADATA_PATH: &str = "metadata.json";

/// A plugin definition stored in the Quill file format.
pub struct PluginFile<'a> {
    module: Cow<'a, [u8]>,
    metadata: PluginMetadata,
}

impl<'a> PluginFile<'a> {
    pub fn new(module: impl Into<Cow<'a, [u8]>>, metadata: PluginMetadata) -> Self {
        Self {
            module: module.into(),
            metadata,
        }
    }

    /// Returns the plugin's module.
    ///
    /// If the plugin is a WebAssembly plugin,
    /// then this is the WASM bytecode.
    ///
    /// If the plugin is a native plugin, then
    /// this is the contents of the shared library
    /// containing the plugin.
    pub fn module(&self) -> &[u8] {
        &*self.module
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
        self.write_module(&mut archive_builder).unwrap();

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

    fn write_module(&self, archive_builder: &mut tar::Builder<impl Write>) -> anyhow::Result<()> {
        let mut header = Header::new_gnu();
        header.set_size(self.module.len() as u64);
        header.set_mode(0o644);

        let path = get_module_path(&self.metadata.target)?;

        archive_builder
            .append_data(&mut header, path, Cursor::new(self.module()))
            .expect("write to Vec failed");

        Ok(())
    }
}

impl PluginFile<'static> {
    /// Deserializes a plugin file.
    pub fn decode(data: impl Read) -> anyhow::Result<Self> {
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(data));

        // Current limitation: the metadata must appear
        // in the tarball before the WASM module.

        let mut metadata = None;
        let mut module = None;
        let mut module_path = None;
        for entry in archive.entries()? {
            let entry = entry?;

            match &*entry.path()?.to_string_lossy() {
                s if s == METADATA_PATH => {
                    let meta = Self::decode_metadata(entry)?;
                    module_path = Some(get_module_path(&meta.target)?);
                    metadata = Some(meta);
                }
                s if Some(s) == module_path => module = Some(Self::decode_wasm_bytecode(entry)?),
                _ => (),
            }
        }

        let metadata =
            metadata.ok_or_else(|| anyhow!("missing plugin metadata ({})", METADATA_PATH))?;
        let module = module
            .ok_or_else(|| anyhow!("missing module ({:?})", module_path))?
            .into();
        Ok(Self { module, metadata })
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

fn get_module_path(target: &PluginTarget) -> anyhow::Result<&'static str> {
    Ok(match target {
        PluginTarget::Wasm => "module.wasm",
        PluginTarget::Native { target_triple } => match target_triple.operating_system {
            OperatingSystem::Linux => "module.so",
            OperatingSystem::Darwin => "module.dylib",
            OperatingSystem::Windows => "module.dll",
            os => bail!("unsupported plugin operating system {:?}", os),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_wasm() {
        let module = vec![0xFF; 1024];
        let metadata = PluginMetadata {
            name: "TestPlugin".to_owned(),
            identifier: "test-plugin".to_owned(),
            version: "0.1.0".to_owned(),
            api_version: "0.1.0".to_owned(),
            description: Some("test plugin".to_owned()),
            authors: vec!["caelunshun".to_owned()],
            target: PluginTarget::Wasm,
        };
        let file = PluginFile::new(module.clone(), metadata.clone());
        let encoded = file.encode(8);
        let decoded = PluginFile::decode(Cursor::new(encoded)).unwrap();

        assert_eq!(decoded.metadata(), &metadata);
        assert_eq!(decoded.module(), module);
    }

    #[test]
    fn roundtrip_native() {
        let module = vec![0xEE; 1024];
        let metadata = PluginMetadata {
            name: "TestPlugin".to_owned(),
            identifier: "test-plugin".to_owned(),
            version: "0.1.0".to_owned(),
            api_version: "0.1.0".to_owned(),
            description: Some("test plugin".to_owned()),
            authors: vec!["caelunshun".to_owned()],
            target: PluginTarget::Native {
                target_triple: Triple::host(),
            },
        };
        let file = PluginFile::new(module.clone(), metadata.clone());
        let encoded = file.encode(8);
        let decoded = PluginFile::decode(Cursor::new(encoded)).unwrap();

        assert_eq!(decoded.metadata(), &metadata);
        assert_eq!(decoded.module(), module);
    }
}
