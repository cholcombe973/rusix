// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
