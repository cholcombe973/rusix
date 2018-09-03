// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// rusixd server daemon
extern crate api;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod lib;
mod pipeline;

use std::collections::HashMap;
use std::fs::File;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{App, Arg};
use lib::config::Peer;
use pipeline::{protocols::client::Client, protocols::server::Server, Value};
use simplelog::{Config, TermLogger};

fn main() {
    let matches = App::new("Rusix")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Distributed posix storage")
        .arg(
            Arg::with_name("client")
                .help("Client mode")
                .long("client")
                .takes_value(false)
                .required(false),
        ).arg(
            Arg::with_name("configfile")
                .default_value("/etc/rusix/config.json")
                .help("The config file")
                .long("configfile")
                .short("c")
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("logfile")
                .default_value("/var/log/rusix")
                .help("File to log to")
                .long("logfile")
                .short("l")
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        ).get_matches();
    let level = match matches.occurrences_of("v") {
        0 => simplelog::LevelFilter::Info, //default
        1 => simplelog::LevelFilter::Debug,
        _ => simplelog::LevelFilter::Trace,
    };
    let logfile = matches.value_of("logfile").unwrap();
    if !Path::new(logfile).exists() {
        File::create(logfile).expect("Creating log file failed");
    }
    TermLogger::init(level, Config::default()).unwrap();
    /*
    WriteLogger::init(
        level,
        Config::default(),
        OpenOptions::new().append(true).open(logfile).unwrap(),
    ).unwrap();
    let config = match File::open(matches.value_of("configfile").unwrap()) {
        Ok(f) => f,
        Err(e) => {
            error!("Unable to open config file: {}", e);
            return;
        }
    };
    */
    if matches.is_present("client"){
        info!("rusixc starting");
        let h: HashMap<String, Value> = HashMap::new();
        let c = Client::new("client1", &h, vec![]);
        // Tell the client about the server layout
        // and create a sample io operation to send
        let peer = Peer{ip: IpAddr::from_str("127.0.0.1").unwrap(), port: 5570};
        let layout = vec![(peer, PathBuf::from("/"))];
        let mut data = api::service::FileOperation::new();
        data.set_fop_req(api::service::Fop::STATFS);
        c.process_fop(layout, &mut data);
    }else {
        info!("rusixd starting");
        let h: HashMap<String, Value> = HashMap::new();

        let s = Server::new(&h, vec![]);
        s.start();
    }
}
