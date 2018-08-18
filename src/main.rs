// rusixd server daemon
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod lib;
mod pipeline;

use std::collections::HashMap;

use pipeline::{protocols::server::Server, Value};
use simplelog::{Config, TermLogger};

fn main() {
    TermLogger::init(simplelog::LogLevelFilter::Debug, Config::default()).unwrap();
    let h: HashMap<String, Value> = HashMap::new();
    let s = Server::new(&h, vec![]);
}
