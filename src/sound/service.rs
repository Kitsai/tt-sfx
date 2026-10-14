use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{error::TtsfxResult, files::FilesService};

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};

pub struct SoundService {
    _files_service: Arc<FilesService>,
    _sink: MixerDeviceSink,
    active: Mutex<Vec<Player>>,
}

impl SoundService {
    pub fn new(file_service: Arc<FilesService>) -> Self {
        let _sink =
            DeviceSinkBuilder::open_default_sink().expect("should open default audio output");
        Self {
            _files_service: file_service,
            _sink,
            active: Mutex::new(Vec::new()),
        }
    }

    pub fn play_from_path(&self, path: &Path) -> TtsfxResult<()> {
        let reader = FilesService::reader_from_path(path);

        let player = rodio::play(self._sink.mixer(), reader?)?;
        self.active.lock().unwrap().push(player);
        Ok(())
    }

    pub fn play_from_data(&self, name: &str) -> TtsfxResult<()> {
        let reader = self._files_service.reader_from_data(name)?;

        let player = rodio::play(self._sink.mixer(), reader)?;
        self.active.lock().unwrap().push(player);
        Ok(())
    }
}
