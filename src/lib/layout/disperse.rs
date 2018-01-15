extern crate rendezvous_hash;

use std::hash::Hash;
use std::path::PathBuf;

use self::rendezvous_hash::{Node, NodeHasher};
use super::super::config::Peer;

/// Store files across a set of servers and paths.  Disperse 
/// will erasure code a file and distribute it across X Peers
/// and Paths while also storing redundancy pieces of the file
/// on X Peers and Paths
pub struct Disperse {
    entry: Vec<(Peer, PathBuf)>,
    redundancy: Vec<(Peer, PathBuf)>,
}

impl Node for Disperse {
    /// Depending on the amount of replicas this will be 1 more more entries
    type NodeId = Vec<(Peer, PathBuf)>;
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
