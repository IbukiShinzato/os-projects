use std::{sync::mpsc::channel, thread, time::Duration};
use threadpool::ThreadPool;

fn main() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for i in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            let message = format!("{:?} send number is {}", thread::current().id(), i);
            tx.send(message)
                .expect("channel will be there waiting for the pool");
            thread::sleep(Duration::from_secs(1));
        });
    }

    for recv in rx.iter().take(n_jobs) {
        println!("{}", recv);
    }
}
