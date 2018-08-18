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
