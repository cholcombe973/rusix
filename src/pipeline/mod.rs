// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate api;
extern crate nix;

pub mod cluster;
pub mod features;
pub mod performance;
pub mod protocols;

use std::collections::HashMap;
use std::fmt::Debug;

use self::api::service_generated::*;
use self::nix::errno::Errno;

/*
How do streaming operations work?
Could the Actix framework help make some of this work easier?
Delegation is handled by the actors and this could handle the hashing
and sockets.
*/

/*
Each rusix process is made of 'plugins' (PipelinePlugin) stacked on 
top of each other in a particular fashion to form a 'graph'.
1. A PipelinePlugin can be present on the client side or server side or both.
2. Every File Op issued by the application (create, write, read etc.)  passes through 
each of the plugins before hitting the disk.
3. The PipelinePlugin can do appropriate things to the FOP or just pass it 
down to the next plugin.

Plugin stack for client:
1. debug/io-stats
2. protocols/client

Plugin stack for server:
1. protocols/server
2. debug/io-stats
3. performance/io-threads
4. cluster/distribute
5. cluster/replicate
6. features/locks
7. features/access-control
8. storage/posix

All modification FOPs (create, write, delete etc.) happen inside a 
5-stage transaction:
1. Lock
2. Pre-op – set a dirty xattr* on the file
3. Write
4. Post-op – clear the dirty xattr* and set pending xattrs* for failed writes.
5. Unlock
*/

pub enum Value {
    Bool(bool),
    UnsignedNumber(u64),
    SignedNumber(i64),
    String(String),
}

pub trait PipelinePlugin {
    /// Display your name
    fn name(&self) -> &str;

    /// Initialize
    fn init(&self, options: HashMap<String, Value>, subvolumes: Vec<String>);

    /// Process data and make available for the next plugin
    /*
    fn process(
        &self,
        io_type: &Fop,
        data: &mut FileOperation,
    ) -> Result<(&Fop, &mut FileOperation), String>;
    */

    /// Stop
    fn stop(&self);
}

/*
pub fn process_pipeline<T>(
    io_type: &Fop,
    data: &mut FileOperation,
    pipeline: &Vec<T>,
) -> Result<(), String>
where
    T: Debug + PipelinePlugin,
{
    // Processing can transform the FileOperation as needed
    // All PipelinePlugins are ephemeral and will be
    // cleaned up by Rust once they go out of scope
    // Client and Server are separate structs that run
    // in their own threads to maintain connections
    // TODO: Remove this expect() for a Result
    let sum = pipeline.iter().fold((io_type, data), |data, ref plugin| {
        plugin
            .process(data.0, data.1)
            .expect(&format!("plugin {} failed", plugin.name()))
    });
    Ok(())
}

*/