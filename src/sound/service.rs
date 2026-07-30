use std::{path::Path, sync::Arc};

use crate::{files::FilesService, result::TtSfxResult};

use rodio::{DeviceSinkBuilder, MixerDeviceSink};

pub struct SoundService {
    _files_service: Arc<FilesService>,
    _sink: MixerDeviceSink,
}

impl SoundService {
    pub fn new(file_service: Arc<FilesService>) -> Self {
        let _sink =
            DeviceSinkBuilder::open_default_sink().expect("should open default audio output");
        Self {
            _files_service: file_service,
            _sink,
        }
    }

    pub fn play_from_path(&self, path: &Path) -> TtSfxResult<()> {
        let reader = FilesService::reader_from_path(path);

        rodio::play(self._sink.mixer(), reader?)?.detach();

        Ok(())
    }
}
