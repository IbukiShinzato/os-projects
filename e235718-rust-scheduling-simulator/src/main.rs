use std::env::args;

mod schedule;
mod thread;

fn main() {
    let args: Vec<String> = args().collect();

    // task count
    let count = 30;

    let algorithm = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or_else(|| panic!("Scheduling algorithm not specified"));

    match algorithm {
        // first in first out
        "fifo" => {
            schedule::fifo_schedulor(count, "./data/fifo_data.csv");
        }
        // shortest job first
        "sjf" => {
            schedule::shortest_job_first(count, "./data/sjf_data.csv");
        }
        // round robin
        "round_robin" => {
            schedule::round_robin(count, 100000, "./data/round_robin_data.csv");
        }
        // ratemonotonic
        "rms" => {
            schedule::rate_monotonic(count);
        }
        "thread" => {
            // thread::run_thread();
        }
        _ => panic!("Unknown algorithm specified"),
    }
}
