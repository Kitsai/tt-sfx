use std::{path::Path, sync::Arc};

use eframe::egui;
use tracing::{error, level_filters::LevelFilter};

use crate::{files::FilesService, sound::SoundService};

pub struct TtSfxApp {
    _file_service: Arc<FilesService>,
    _sound_service: SoundService,
}

impl eframe::App for TtSfxApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Teste").clicked() {
                if let Err(e) = self
                    ._sound_service
                    .play_from_path(Path::new("resources\\pf.mp3"))
                {
                    error!("Failed to play test sound: {}", e);
                }
            }
        });
    }
}

impl TtSfxApp {
    pub fn run() -> eframe::Result {
        let options = eframe::NativeOptions::default();
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::INFO)
            .init();
        eframe::run_native("TtSfx", options, Box::new(|cc| Ok(Box::new(Self::new(cc)))))
    }
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _file_service = Arc::new(FilesService::new());
        let _sound_service = SoundService::new(_file_service.clone());

        Self {
            _file_service,
            _sound_service,
        }
    }
}
