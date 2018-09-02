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
use std::net::IpAddr;
use std::path::PathBuf;

use self::rendezvous_hash::{Node, NodeHasher};
use super::super::config::Peer;

/// Store files across servers and backend paths.  There is no data redundancy
/// when using this
#[derive(Clone, Debug)]
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

#[test]
fn test_distribute() {
    use self::rendezvous_hash::RendezvousNodes;
    use std::net::{IpAddr, Ipv4Addr};

    let e1 = Distribute {
        entry: (
            Peer {
                ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                port: 8888,
            },
            PathBuf::from("/mnt/sda"),
        ),
    };
    let e2 = Distribute {
        entry: (
            Peer {
                ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                port: 8888,
            },
            PathBuf::from("/mnt/sdb"),
        ),
    };
    let mut nodes = RendezvousNodes::default();
    nodes.insert(e1.clone());
    nodes.insert(e2.clone());

    // FIX ME This is not a perfect distribution
    // This should correspond to replica set e1
    {
        let r1 = &nodes.calc_candidates(&"hello").next().unwrap();
        assert_eq!(&r1.entry, &e2.entry);
    }

    // This should correspond to replica set e2
    {
        let r2 = nodes.calc_candidates(&"key_foo").next().unwrap();
        assert_eq!(&r2.entry, &e2.entry);
    }
}
