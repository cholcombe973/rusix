extern crate nix;

mod cluster;
mod features;
mod performance;
mod protocols;

//use self::cluster::{distribute, replicate};

use std::collections::HashMap;
use std::fmt::Debug;

use self::nix::Errno;

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
    fn init(&self, options: HashMap<String,Value>, subvolumes: Vec<String>);

    /// Process data and make available for the next plugin
    fn process(&self, name: &str, data: &mut [u8]) -> Result<(&str, &mut [u8]), String>;

    /// Stop
    fn stop(&self);
}

pub fn process_pipeline<T>(name: &str, mut data: &mut [u8], pipeline: &Vec<T>) -> Result<(), String>
where
    T: Debug + PipelinePlugin,
{
    for p in pipeline {
        debug!("Processing data with {:?}", p);
        p.process(name, &mut data)?;
    }

    Ok(())
}
