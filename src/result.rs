use std::{fs, io};

use rodio::PlayError;
use thiserror::Error;

pub type TtSfxResult<T> = Result<T, TtSfxError>;

#[derive(Error, Debug)]
pub enum TtSfxError {
    #[error("Error opening file: {0}")]
    FileAccess(#[from] io::Error),
    #[error("Could not play audio: {0}")]
    AudioPlay(#[from] PlayError),
}
