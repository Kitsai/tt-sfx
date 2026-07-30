use std::path::Path;

use eframe::egui;
use tracing::level_filters::LevelFilter;

pub struct TtSfxApp {
    audio_sink: rodio::MixerDeviceSink,
}

impl eframe::App for TtSfxApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Teste").clicked() {
                self.play_sound(Path::new("resources\\pf.mp3"));
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
        let audio_sink =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio output");
        Self { audio_sink }
    }

    fn play_sound(&self, path: &Path) {
        let file = std::fs::File::open(path).expect("open sfx file");
        rodio::play(self.audio_sink.mixer(), std::io::BufReader::new(file))
            .expect("decode + play sfx")
            .detach();
    }
}
