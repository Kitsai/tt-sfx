use std::{
    io::Write,
    path::{Path, PathBuf},
};

use blake3::{Hash, Hasher};
use tempfile::NamedTempFile;

use crate::result::TtSfxResult;

pub struct AssetWriter {
    tmp: NamedTempFile,
    hasher: Hasher,
    final_path: PathBuf,
}

impl AssetWriter {
    pub fn begin(dir: &Path, name: &str) -> TtSfxResult<Self> {
        std::fs::create_dir_all(dir)?;
        Ok(Self {
            tmp: NamedTempFile::new_in(dir)?,
            hasher: Hasher::new(),
            final_path: dir.join(name),
        })
    }

    pub fn write_chunk(&mut self, chunk: &[u8]) -> TtSfxResult<()> {
        self.tmp.write_all(chunk)?;
        self.hasher.update(chunk);
        Ok(())
    }

    pub fn finish(self) -> TtSfxResult<Hash> {
        self.tmp.persist(&self.final_path)?;
        Ok(self.hasher.finalize())
    }
}
