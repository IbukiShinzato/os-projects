use std::sync::{Arc, Mutex};

use crate::server::Server;

pub struct Client {
    server1: Arc<Mutex<Server>>,
    server2: Arc<Mutex<Server>>,
}

impl Client {
    pub fn new(hotel: Arc<Mutex<Server>>, plane: Arc<Mutex<Server>>) -> Self {
        if hotel.lock().unwrap().get_name() == String::from("hotel") {
            Self {
                server1: hotel,
                server2: plane,
            }
        } else {
            Self {
                server1: plane,
                server2: hotel,
            }
        }
    }

    pub fn run(&self) {
        let mut server1 = self.server1.lock().unwrap();
        server1.lock();

        let mut server2 = self.server2.lock().unwrap();
        server2.lock();

        server1.reserve();
        server2.reserve();

        server1.release();
        server2.release();

        println!("Reserve completed");
    }
}
