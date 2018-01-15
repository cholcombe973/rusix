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

#[derive(Debug, Copy, Clone)]
struct Node {
    id: usize,
    addr: SocketAddr,
}

impl Node {
    fn new(ip: &str, port: u16, id: usize) -> Self {
        let addr = SocketAddr::new(IpAddr::from_str(&ip).unwrap(), port);
        Node { id: id, addr: addr }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        format!("{}|{}", self.addr, self.id)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id && self.addr == other.addr
    }
}

