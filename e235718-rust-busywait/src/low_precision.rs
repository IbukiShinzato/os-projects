use mouse_position::mouse_position::Mouse;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::recorder::MouseRecorder;

pub fn record_low_precision(recorder: Arc<Mutex<MouseRecorder>>) {
    thread::spawn(move || {
        while let Ok(mut recorder) = recorder.lock() {
            // get mouse coordinate
            let position = Mouse::get_mouse_position();
            let (x, y) = match position {
                Mouse::Position { x, y } => (x as f64, y as f64),
                Mouse::Error => panic!("Error getting mouse position"),
            };
            let now = recorder.record(x, y);
            println!("{},{},{}", now, x, y);
            drop(recorder);
            thread::sleep(Duration::from_secs(1));
        }
    });
}
