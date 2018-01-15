extern crate rendezvous_hash;

use std::hash::Hash;
use std::net::IpAddr;
use std::path::PathBuf;

use self::rendezvous_hash::{Node, NodeHasher};
use super::super::config::Peer;

pub struct Distribute {
    entry: (Peer, PathBuf),
}

impl Node for Distribute {
    type NodeId = (Peer, PathBuf);
    type HashCode = u64;

    fn node_id(&self) -> &Self::NodeId {
        &self.entry
    }
    fn hash_code<H, U: Hash>(&self, hasher: &H, item: &U) -> Self::HashCode
    where
        H: NodeHasher<Self::NodeId>,
    {
        hasher.hash(&self.entry, item)
    }
}
