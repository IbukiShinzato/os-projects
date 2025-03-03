use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{self, Write};

pub struct MouseRecorder {
    records: Vec<(DateTime<Local>, f64, f64)>,
}

impl MouseRecorder {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn record(&mut self, x: f64, y: f64) -> DateTime<Local> {
        let now = Local::now();
        self.records.push((now, x, y));
        now
    }

    pub fn save_to_csv(&self, filename: &str, precision_mode: bool) -> io::Result<()> {
        // file separately
        let filename = match precision_mode {
            true => format!("data/high_precision_{}", filename),
            false => format!("data/low_precision_{}", filename),
        };

        let mut file = File::create(filename)?;
        writeln!(file, "timestamp,x,y")?;
        for &(timestamp, x, y) in &self.records {
            writeln!(file, "{},{},{}", timestamp, x, y)?;
        }
        Ok(())
    }
}
