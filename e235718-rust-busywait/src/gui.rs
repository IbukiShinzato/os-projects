use eframe::{egui, App};
use std::sync::{Arc, Mutex};

use crate::high_precision::record_high_precision;
use crate::low_precision::record_low_precision;
use crate::recorder::MouseRecorder;
use crate::replay::replay_from_csv;

pub fn run() {
    let app = MouseRecorderApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mouse Recorder",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .expect("Failed to run");
}

struct MouseRecorderApp {
    recorder: Arc<Mutex<MouseRecorder>>,
    recording: bool,

    // true  => high precision
    // false => low precision
    precision_mode: bool,
}

impl Default for MouseRecorderApp {
    fn default() -> Self {
        Self {
            recorder: Arc::new(Mutex::new(MouseRecorder::new())),
            recording: false,
            precision_mode: true,
        }
    }
}

impl App for MouseRecorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Recorder");

            if ui.button("Start Recording").clicked() {
                if !self.recording {
                    self.recording = true;
                    let recorder = Arc::clone(&self.recorder);
                    let precision_mode = self.precision_mode;

                    if precision_mode {
                        // high efficiency
                        record_high_precision(recorder);
                    } else {
                        // low efficiency
                        record_low_precision(recorder);
                    }
                }
            }

            if ui.button("Stop Recording").clicked() {
                self.recording = false;
            }

            if ui.button("Save to CSV").clicked() {
                self.recorder
                    .lock()
                    .unwrap()
                    .save_to_csv("mouse_data.csv", self.precision_mode)
                    .unwrap();
            }

            ui.checkbox(&mut self.precision_mode, "High Precision Mode");

            if ui.button("Replay to CSV").clicked() {
                replay_from_csv("mouse_data.csv").expect("Failed to replay");
            }
        });
    }
}
