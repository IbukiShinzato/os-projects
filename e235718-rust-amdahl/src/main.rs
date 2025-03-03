use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n = 10_000_000;
    let mut csv_file = File::create("./data/cpu4_results.csv").expect("Failed to create file");
    writeln!(csv_file, "ParallelizationRatio,Worker,Time(ms),Speedup")
        .expect("Failed to write header");

    // single thread real time
    let serial_time = {
        let start_time = Instant::now();
        for i in 3..=n {
            _ = is_prime(i);
        }
        start_time.elapsed().as_millis() as f64
    };

    // select number of cpu
    let cpu = 4;

    for n_workers in 1..=cpu {
        let pool = ThreadPool::new(n_workers);

        for parallelization_ratio in (0..=100).step_by(10) {
            let m = (n as f64 * (parallelization_ratio as f64 / 100.0)) as usize;
            println!("{}", m);
            let non_parallel_range = (m + 1)..=n;

            let start_time = Instant::now();

            let results = Arc::new(Mutex::new(0));
            for i in 3..=m {
                let results = Arc::clone(&results);
                pool.execute(move || {
                    if is_prime(i) {
                        *results.lock().unwrap() += 1;
                    }
                });
            }
            pool.join();

            for i in non_parallel_range {
                _ = is_prime(i);
            }

            // multithread and single thread time
            let elapsed_time = start_time.elapsed().as_millis() as f64;

            // rate of acceleration
            let speedup = serial_time / elapsed_time;

            writeln!(
                csv_file,
                "{},{},{},{:.2}",
                parallelization_ratio, n_workers, elapsed_time, speedup
            )
            .expect("Failed to write to file");
        }
    }
}
