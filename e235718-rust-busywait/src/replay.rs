use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

pub fn replay_from_csv(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut last_timestamp: Option<u128> = None;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 || parts[0] == "timestamp" {
            continue;
        }

        let timestamp: u128 = parts[0].parse().unwrap_or(0);
        let x: f64 = parts[1].parse().unwrap_or(0.0);
        let y: f64 = parts[2].parse().unwrap_or(0.0);

        if let Some(last) = last_timestamp {
            let delay = timestamp.saturating_sub(last);
            thread::sleep(Duration::from_millis(delay as u64));
        }
        last_timestamp = Some(timestamp);

        draw_point(x, y);
    }

    Ok(())
}

fn draw_point(x: f64, y: f64) {
    println!("Drawing point at ({}, {})", x, y);
}
