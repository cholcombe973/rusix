extern crate rendezvous_hash;

use std::hash::Hash;
use std::path::PathBuf;

use self::rendezvous_hash::{Node, NodeHasher};
use super::super::config::Peer;

/// Store files across a set of servers and paths.  Replicate will copy
/// a file X number of times to ensure data redundancy.
#[derive(Clone, Debug)]
pub struct Replicate {
    entry: Vec<(Peer, PathBuf)>,
}

impl Node for Replicate {
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

#[test]
fn test_replicate() {
    use self::rendezvous_hash::RendezvousNodes;
    use std::net::{IpAddr, Ipv4Addr};
    // One 2x replica set
    let e1 = Replicate {
        entry: vec![
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sda"),
            ),
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sda"),
            ),
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sda"),
            ),
        ],
    };
    let e2 = Replicate {
        entry: vec![
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sdb"),
            ),
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sdb"),
            ),
            (
                Peer {
                    ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)),
                    port: 8888,
                },
                PathBuf::from("/mnt/sdb"),
            ),
        ],
    };
    let mut nodes = RendezvousNodes::default();
    nodes.insert(e1.clone());
    nodes.insert(e2.clone());

    // This should correspond to replica set e1
    {
        let replica_set_1 = &nodes.calc_candidates(&"hello").next().unwrap();
        assert_eq!(
            replica_set_1.entry,
            vec![
                e1.entry[0].clone(),
                e1.entry[1].clone(),
                e1.entry[2].clone(),
            ],
        );
    }

    // This should correspond to replica set e2
    {
        let replica_set_2 = &nodes.calc_candidates(&"key_foo").next().unwrap();

        assert_eq!(
            replica_set_2.entry,
            vec![
                e2.entry[0].clone(),
                e2.entry[1].clone(),
                e2.entry[2].clone(),
            ],
        );
    }
}
