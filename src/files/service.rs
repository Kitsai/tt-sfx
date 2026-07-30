use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::result::TtSfxResult;

pub struct FilesService {
    pub dirs: FileDirs,
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

    pub fn reader_from_path(path: &Path) -> TtSfxResult<BufReader<File>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }
}

#[derive(Clone)]
struct FileDirs {
    pub data: PathBuf,
    pub cache: PathBuf,
    pub config: PathBuf,
}
