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
extern crate rayon;
extern crate zmq;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use self::api::service_generated::*;
use self::futures_cpupool::CpuPool;
use self::rayon::prelude::*;
use super::super::super::lib::dht::*;
use super::super::super::lib::fop::*;
use super::super::Value;
use lib::config::Peer;

pub struct Client {
    pub peer_name: String,
    dht: Dht,
    pool: CpuPool,
}

// Client sends an RPC request to one or more servers
impl Client {
    pub fn new(name: &str, options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        let pool = CpuPool::new_num_cpus();
        Client {
            dht: Dht::new(None),
            peer_name: name.to_string(),
            pool: pool,
        }
    }

    fn mkdir(&self, parent_id: u128, basename: &str, mode: u32) -> Result<(), String> {
        // Hash the basename
        // Find the parent directory file
        // Write new directory file to server where it should belong to
        // Modify parent directory file and link in new hash
        /*
            client mkdir (involves 2 round trips).  Potentially slow
                basename <-> server 1
                link dir <-> server 2
            
            Could this be done in parallel?  I don't see why not as long
            as we can reverse it if it fails
        */

        let base_server = self.dht.locate_path(&Path::new(basename), 4, 2);
        // Send Fop to that server to make the directory

        let parent_server = self.dht.locate_hash(parent_id, 4, 2);
        // Send Fop to that server to link the directory

        Ok(())
    }

    // The FOP should be processed before being sent by the client
    pub fn process_fop(&self, layout: Vec<(Peer, PathBuf)>, data: &[u8]) -> Result<(), String> {
        // Client is the end of the pipeline.
        // send the Fop over to the server(s)
        let context = zmq::Context::new();

        layout.par_iter().for_each(|entry| {
            // This is likely inefficient because it writes to the heap first
            let client = context.socket(zmq::REQ).expect("socket creation failed");
            client
                .connect(&format!("tcp://{}:{}", entry.0.ip, entry.0.port))
                .expect("failed connecting client");
            debug!("Sending: {:?} len: {}", data, data.len());
            client
                .send(data, 0)
                .map_err(|e| e.to_string())
                .expect("request failed");
        });

        // Packet sent
        Ok(())
    }

    pub fn stop(&self) {}
}
