// rusixd server daemon
#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::{Config, TermLogger};

fn main() {
    TermLogger::init(simplelog::LogLevelFilter::Debug, Config::default()).unwrap();
}
