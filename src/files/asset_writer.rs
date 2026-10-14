use std::{
    io::Write,
    path::{Path, PathBuf},
};

use blake3::{Hash, Hasher};
use tempfile::NamedTempFile;

use crate::error::{FsResultExt, TtsfxResult};

pub struct AssetWriter {
    tmp: NamedTempFile,
    hasher: Hasher,
    final_path: PathBuf,
}

impl AssetWriter {
    pub fn begin(dir: &Path, name: &str) -> TtsfxResult<Self> {
        std::fs::create_dir_all(dir).fs_context(crate::error::FsOperation::CreateDir, dir)?;
        Ok(Self {
            tmp: NamedTempFile::new_in(dir).fs_context(crate::error::FsOperation::Create, dir)?,
            hasher: Hasher::new(),
            final_path: dir.join(name),
        })
    }

    pub fn write_chunk(&mut self, chunk: &[u8]) -> TtsfxResult<()> {
        self.tmp
            .write_all(chunk)
            .fs_context(crate::error::FsOperation::Write, self.tmp.path())?;
        self.hasher.update(chunk);
        Ok(())
    }

    pub fn finish(self) -> TtsfxResult<Hash> {
        self.tmp
            .persist(&self.final_path)
            .fs_context(crate::error::FsOperation::Rename, &self.final_path)?;
        Ok(self.hasher.finalize())
    }
}
