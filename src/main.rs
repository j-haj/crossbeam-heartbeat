extern crate crossbeam;
use std::thread;
#[macro_use]
use crossbeam_channel as channel;

struct Worker {
    id: u64,
    chan: channel::Sender<String>,
}

impl Worker {
    fn new(id: u64, chan: channel::Sender<String>) -> Worker {
        Worker { id: id, chan: chan }
    }

    fn run(&self) {
        let t = channel::tick(s(1));
        loop {
            select! {
                recv(t) => self.chan.send(format!("Heartbeat from {}", self.id)),
            }
        }
    }
}

fn main() {
    let (sender, receiver) = channel::bounded(5);
    for i in 0..10 {
        thread::spawn(move || Worker::new(i, sender).run());
    }
    loop {
        select! {
            recv(receiver, msg) => match msg {
                Some(m) => println!("Master received: {}"),
                None => println!("error - bad message received"),
            }
        }
    }
}
