// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Sample client

extern crate api;
#[macro_use]
extern crate log;
extern crate simplelog;

//mod lib;
//mod pipeline;

use std::collections::HashMap;

//use pipeline::{protocols::client::Client, Value};
use simplelog::{Config, TermLogger};

fn main() {
    TermLogger::init(simplelog::LevelFilter::Debug, Config::default()).unwrap();
    info!("rusix client starting");
    //let h: HashMap<String, Value> = HashMap::new();

    //let c = Client::new("", &h, vec![]);
}
