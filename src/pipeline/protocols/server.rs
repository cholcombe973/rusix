// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate api;
//extern crate capnp;
extern crate flatbuffers;
extern crate futures;
extern crate futures_cpupool;
extern crate zmq;

use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;

use api::{service_generated::*};
use self::zmq::{Context, Result as ZmqResult, Socket};
use super::super::Value;

pub struct Server {
    // Worker pool
    // pool: CpuPool,
    //frontend: Socket,
    //backend: Socket,
}

// Server receives an RPC request and responds
impl Server {
    // Start the server
    pub fn new(options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        Server {}
    }

    pub fn start(&mut self) -> ZmqResult<()> {
        // Preallocate a receiving buffer
        let context = Context::new();
        let mut frontend = context.socket(zmq::REP).unwrap();
        frontend
            .bind("tcp://*:5570")
            .expect("server failed binding frontend");
        loop {
            // Block until we have events to process
            debug!("waiting for packet");
            let msg = frontend.recv_bytes(0)?;
            self.process_fop(&msg);
            frontend.send(b"thanks", 0)?;
        }
    }

    // This should process the Fop down to posix and then
    // send the result back to the client.
    fn process_fop(&self, data: &[u8]) -> Result<(), String> {
        let start = Instant::now();
        let fop = get_root_as_operation(&data);
        let elapsed = start.elapsed();
        if fop.stat().is_some(){
            debug!("fop: {:?}", fop.stat().unwrap().rfid().unwrap());
        }
        debug!(
            "flatbuffers load msg took: {} nanosecs",
            //elapsed.as_secs(),
            //elapsed.subsec_millis()
            elapsed.subsec_nanos()
        );
        Ok(())
    }

    pub fn stop(&self) {}
}
