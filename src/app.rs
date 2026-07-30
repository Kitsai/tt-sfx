use eframe::egui;

pub struct TtSfxApp {
    name: String,
    age: u32,
}

impl eframe::App for TtSfxApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
        });
    }
}

impl TtSfxApp {
    pub fn run() -> eframe::Result {
        let options = eframe::NativeOptions::default();
        eframe::run_native("TtSfx", options, Box::new(|cc| Ok(Box::new(Self::new(cc)))))
    }
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "TtSfx".to_owned(),
            age: 0,
        }
    }
}
