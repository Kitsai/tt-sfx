use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::{
    error::{FsResultExt, TtsfxResult},
    files::asset_writer::AssetWriter,
};

pub struct FilesService {
    dirs: FileDirs,
}

impl FilesService {
    pub fn new() -> Self {
        let proj_dirs =
            directories::ProjectDirs::from("", "", "TtSfx").expect("could not find home directory");

        let data = proj_dirs.data_dir().to_path_buf();
        let cache = proj_dirs.cache_dir().to_path_buf();
        let config = proj_dirs.config_dir().to_path_buf();

        let dirs = FileDirs {
            data,
            cache,
            config,
        };

        Self { dirs }
    }

    #[cfg(test)]
    fn new_with_dirs(dirs: FileDirs) -> Self {
        Self { dirs }
    }

    pub fn reader_from_path(path: &Path) -> TtsfxResult<BufReader<File>> {
        let file = File::open(path).fs_context(crate::error::FsOperation::Read, path)?;
        Ok(BufReader::new(file))
    }

    pub fn reader_from_data(&self, name: &str) -> TtsfxResult<BufReader<File>> {
        let path = self.dirs.data.join(name);
        Self::reader_from_path(&path)
    }

    pub fn writer_to_data(&self, name: &str) -> TtsfxResult<AssetWriter> {
        AssetWriter::begin(&self.dirs.data, name)
    }

    pub fn reader_from_cache(&self, name: &str) -> TtsfxResult<BufReader<File>> {
        let path = self.dirs.cache.join(name);
        Self::reader_from_path(&path)
    }

    pub fn writer_to_cache(&self, name: &str) -> TtsfxResult<AssetWriter> {
        AssetWriter::begin(&self.dirs.cache, name)
    }

    pub fn reader_from_config(&self, name: &str) -> TtsfxResult<BufReader<File>> {
        let path = self.dirs.config.join(name);
        Self::reader_from_path(&path)
    }

    pub fn writer_to_config(&self, name: &str) -> TtsfxResult<AssetWriter> {
        AssetWriter::begin(&self.dirs.config, name)
    }
}

#[derive(Clone)]
struct FileDirs {
    pub data: PathBuf,
    pub cache: PathBuf,
    pub config: PathBuf,
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    fn service_in(dir: &tempfile::TempDir) -> FilesService {
        let base = dir.path();
        FilesService::new_with_dirs(FileDirs {
            data: base.join("data"),
            cache: base.join("cache"),
            config: base.join("config"),
        })
    }

    #[test]
    fn reads_from_data_dir() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("data")).unwrap();
        std::fs::write(dir.path().join("data/foo.txt"), b"hi").unwrap();

        let sut = service_in(&dir);

        assert!(sut.reader_from_data("foo.txt").is_ok());
    }

    #[test]
    fn missing_file_in_data_errors() {
        let dir = tempdir().unwrap();
        let service = service_in(&dir);
        assert!(service.reader_from_data("missing.txt").is_err());
    }

    #[test]
    fn reads_from_cache_dir() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("cache")).unwrap();
        std::fs::write(dir.path().join("cache/foo.txt"), b"hi").unwrap();

        let sut = service_in(&dir);

        assert!(sut.reader_from_cache("foo.txt").is_ok());
    }

    #[test]
    fn missing_file_in_cache_errors() {
        let dir = tempdir().unwrap();
        let service = service_in(&dir);
        assert!(service.reader_from_cache("missing.txt").is_err());
    }
}
