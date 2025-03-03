mod client;
mod server;

use crate::client::Client;
use crate::server::Server;
use std::thread;

fn main() {
    let hotel = Server::new(String::from("hotel"));
    let plane = Server::new(String::from("plane"));

    let client1 = Client::new(hotel.clone(), plane.clone());
    let client2 = Client::new(plane.clone(), hotel.clone());

    let thread1 = thread::spawn(move || {
        client1.run();
    });

    let thread2 = thread::spawn(move || {
        client2.run();
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!(
        "Plane = {}, Hotel = {}",
        plane.lock().unwrap().count,
        hotel.lock().unwrap().count
    )
}
