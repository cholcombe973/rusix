// rusixd server daemon
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod lib;
mod pipeline;

use std::collections::HashMap;
use std::thread;

use pipeline::{protocols::client::Client, protocols::server::Server, Value};
use simplelog::{Config, TermLogger};

fn main() {
    TermLogger::init(simplelog::LevelFilter::Debug, Config::default()).unwrap();
    let j = thread::spawn(|| {
        let h: HashMap<String, Value> = HashMap::new();
        let s = Server::new(&h, vec![]);
    });
    let h: HashMap<String, Value> = HashMap::new();

    let c = Client::new("", &h, vec![]);

    j.join();
}
