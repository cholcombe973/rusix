// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate api;
extern crate futures;
extern crate futures_cpupool;
extern crate zmq;

use std::collections::HashMap;
//use std::thread;

use self::api::service::*;
use self::futures::Future;
use self::futures_cpupool::CpuPool;
use super::super::Value;

pub struct Server {
    // Worker pool
// pool: CpuPool,
}

// Server receives an RPC request and responds
impl Server {
    // Start the server
    pub fn new(options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        let context = zmq::Context::new();
        let mut frontend = context.socket(zmq::ROUTER).unwrap();
        frontend
            .bind("tcp://*:5570")
            .expect("server failed binding frontend");
        let mut backend = context.socket(zmq::DEALER).unwrap();
        backend
            .bind("inproc://backend")
            .expect("server failed binding backend");
        zmq::proxy(&mut frontend, &mut backend).expect("server failed proxying");
        Server {}
    }

    // This should process the Fop down to posix and then
    // send the result back to the client.
    pub fn process_fop(&self, io_type: &Fop, data: &mut FileOperation) -> Result<(), String> {
        debug!("fop: {:?}", io_type);
        Ok(())
    }

    pub fn stop(&self) {}
}
