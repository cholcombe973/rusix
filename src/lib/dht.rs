// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
    There are no special nodes with special knowledge of where files are or should be.
    Directories exist on all subvolumes (bricks or lower-level aggregations of bricks); 
    files exist on only one.
    Files are assigned to subvolumes based on consistent hashing, 
    and even more specifically a form of consistent hashing exemplified by Amazon's Dynamo.
*/
extern crate rendezvous_hash;

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use self::rendezvous_hash::RendezvousNodes;

pub struct Dht {
    hasher: RendezvouNodes,
}

impl Dht {
    pub fn new() -> Self {
        Dht {
            hasher: RendezvousNodes::default(),
        }
    }
}
