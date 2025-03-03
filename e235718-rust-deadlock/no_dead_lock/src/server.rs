use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

use regex::Regex;

pub struct Server {
    pub count: usize,
    lock: Arc<Mutex<bool>>,
    condvar: Condvar,
    name: String,
}

impl Server {
    pub fn new(name: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Server {
            count: 0,
            lock: Arc::new(Mutex::new(false)),
            condvar: Condvar::new(),
            name: name,
        }))
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn reserve(&mut self) {
        println!("server-enter: count={}", self.count);
        self.count += 1;
        println!("server-leave: count={}", self.count);
    }

    pub fn lock(&self) {
        let mut lock = self.lock.lock().unwrap();

        while *lock {
            let thread_id = format!("{:?}", thread::current().id());
            let re = Regex::new(r"\d+").unwrap();
            if let Some(captures) = re.captures(&thread_id) {
                let id_number = captures.get(0).unwrap().as_str();
                println!(
                    "Thread {} is waiting for the lock to be released...",
                    id_number
                );
            }
            lock = self.condvar.wait(lock).unwrap();
        }

        *lock = true;
    }

    pub fn release(&self) {
        let mut lock = self.lock.lock().unwrap();
        *lock = false;
        println!("lock released");
        self.condvar.notify_one();
    }
}
