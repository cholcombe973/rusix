// rusixd server daemon
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod lib;
mod pipeline;

use simplelog::{Config, TermLogger};

fn main() {
    TermLogger::init(simplelog::LogLevelFilter::Debug, Config::default()).unwrap();
}
