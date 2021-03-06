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
extern crate protobuf;
extern crate rayon;
extern crate zmq;

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;

use self::api::service::*;
use self::futures::Future;
use self::futures_cpupool::CpuPool;
use self::protobuf::Message;
use self::rayon::prelude::*;
use super::super::Value;
use lib::config::Peer;

pub struct Client {
    pub peer_name: String,
    pool: CpuPool,
}

// Client sends an RPC request to one or more servers
impl Client {
    pub fn new(name: &str, options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        let pool = CpuPool::new_num_cpus();
        Client {
            peer_name: name.to_string(),
            pool: pool,
        }
    }

    // The FOP should be processed before being sent by the client
    pub fn process_fop(
        &self,
        layout: Vec<(Peer, PathBuf)>,
        io_type: &Fop,
        data: &mut FileOperation,
    ) -> Result<(), String> {
        // Client is the end of the pipeline.
        // send the Fop over to the server(s)
        let context = zmq::Context::new();

        layout.par_iter().for_each(|entry| {
            // This is likely inefficient because it writes to the heap first
            let payload = data.write_to_bytes().expect("Message serialization failed");
            let client = context.socket(zmq::REQ).expect("socket creation failed");
            client
                .set_identity(self.peer_name.as_bytes())
                .expect("failed setting client id");
            client
                .connect(&format!("tcp://{}:{}", entry.0.ip, entry.0.port))
                .expect("failed connecting client");
            client
                .send(&payload, 0)
                .map_err(|e| e.to_string())
                .expect("request failed");
        });

        // Packet sent
        Ok(())
    }

    pub fn stop(&self) {}
}
