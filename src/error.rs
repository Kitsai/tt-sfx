use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TtsfxError {
    #[error("Failed to {operation} {path:?}: {source}")]
    Fs {
        operation: FsOperation,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Graphics/window error: {0}")]
    EFrame(#[from] eframe::Error),
    #[error("Failed to play sound: {0}")]
    Playback(#[from] rodio::PlayError),
    #[error("Audio output error: {0}")]
    AudioDevice(#[from] rodio::stream::DeviceSinkError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsOperation {
    Read,
    Write,
    Create,
    CreateDir,
    Delete,
    RemoveDir,
    Rename,
    Copy,
}

impl Display for FsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Create => "create",
            Self::CreateDir => "create directory",
            Self::Delete => "delete",
            Self::RemoveDir => "remove directory",
            Self::Rename => "rename",
            Self::Copy => "copy",
        };
        write!(f, "{s}")
    }
}

pub type TtsfxResult<T, E = TtsfxError> = Result<T, E>;

pub trait FsResultExt<T> {
    fn fs_context(self, operation: FsOperation, path: &Path) -> TtsfxResult<T>;
}

impl<T, E: Into<io::Error>> FsResultExt<T> for Result<T, E> {
    fn fs_context(self, operation: FsOperation, path: &Path) -> TtsfxResult<T> {
        self.map_err(|source| TtsfxError::Fs {
            operation,
            path: path.to_owned(),
            source: source.into(),
        })
    }
}
