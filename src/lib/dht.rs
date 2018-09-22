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
extern crate twox_hash;
extern crate uuid;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use self::twox_hash::XxHash;
use self::uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bucket {
    pub ip: IpAddr,
    pub start_range: u64,
    pub end_range: u64,
    //pub commit_hash: u64,
    pub weight: u8,
}

#[derive(Debug)]
pub struct Dht {
    seed: u64,
    buckets: Vec<Bucket>,
}

impl Dht {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(200);
        Dht {
            seed,
            buckets: vec![],
        }
    }

    /// Add a new node into the hash table with a given
    /// weight.  The weight represents how much data
    /// a given node should hold compared to the rest
    /// of the nodes.
    pub fn add_node(&mut self, ip: IpAddr, weight: u8) {
        // Add node to hash range, possibly splitting
        // an existing range
        // Nodes must be inserted in sorted order
        if self.buckets.len() == 0 {
            self.buckets.push(Bucket {
                ip,
                start_range: u64::min_value(),
                end_range: u64::max_value(),
                weight,
            });
        } else {
            // We need to locate where this should be inserted
            // There's a few different options here
            // 1. We could split all the segments and require some reshuffling on all the nodes
            // 2. We could split a particular segment and end up shuffling files on a subset of nodes
            //    but have an imbalance resulting from that.

            // This will do a partial reshuffle on all nodes
            let segment_length = u64::max_value() / (self.buckets.len() as u64 + 1);

            // Start it out with blank values
            self.buckets.push(Bucket {
                ip,
                start_range: 0,
                end_range: 0,
                weight,
            });

            // Change all the segment lengths which frees up room
            let mut i = 0;
            for bucket in &mut self.buckets {
                if i == 0 {
                    bucket.start_range = segment_length * i as u64;
                    bucket.end_range = segment_length * (i as u64 + 1);
                } else {
                    bucket.start_range = (segment_length * i) + 1 as u64;
                    bucket.end_range = segment_length * (i as u64 + 1);
                }
                i += 1;
            }
        }

        // Keep the buckets sorted
        self.buckets.sort_by_key(|b| b.start_range);
    }

    /// Remove a node from the hash table with the given
    /// weight.
    pub fn remove_node(&mut self, ip: IpAddr) {
        // Remove node from the hash range, possibly
        // joining a split range back together
        // Nodes must be removed in sorted order

        if self.buckets.len() == 0 {
            return;
        } else {
            // We need to locate where this should be removed
            // There's a few different options here
            // 1. We could split all the segments and require some reshuffling on all the nodes
            // 2. We could split a particular segment and end up shuffling files on a subset of nodes
            //    but have an imbalance resulting from that.

            // Retain all buckets that don't match this ip
            self.buckets.retain(|b| b.ip != ip);

            // This will do a partial reshuffle on all nodes
            let segment_length = u64::max_value() / (self.buckets.len() as u64 );

            // Change all the segment lengths
            let mut i: u64 = 0;
            for bucket in &mut self.buckets {
                if i == 0 {
                    bucket.start_range = segment_length * i as u64;
                    bucket.end_range = segment_length * (i as u64 + 1);
                } else {
                    bucket.start_range = (segment_length * i) + 1 as u64;
                    bucket.end_range = segment_length * (i as u64 + 1);
                }
                i += 1;
            }
        }

        // Keep the buckets sorted
        self.buckets.sort_by_key(|b| b.start_range);
    }

    /// Find the node that should contain the uuid
    pub fn locate(&self, id: &Uuid) -> Option<Bucket> {
        let start = Instant::now();
        if self.buckets.len() == 0 {
            return None;
        }
        // Binary search through buckets to find
        // the one containing the correct hash range
        let mut h = XxHash::with_seed(200);
        h.write(id.as_bytes());
        // erasure coded m+n count
        h.write_u8(3);
        let hash = h.finish();
        debug!("hash: {}", hash);
        // Search for the containing bucket
        let res = self.buckets.binary_search_by(|bucket| {
            if hash < bucket.start_range {
                debug!("{} > {}", bucket.start_range, hash);
                Ordering::Greater
            } else if hash >= bucket.start_range && hash <= bucket.end_range {
                debug!("{} == {}", bucket.start_range, hash);
                Ordering::Equal
            } else {
                debug!("{} < {}", bucket.start_range, hash);
                Ordering::Less
            }
        });
        let elapsed = start.elapsed();
        println!("dht locate took {} nanosecs", elapsed.subsec_nanos());
        match res {
            Ok(idx) => Some(self.buckets[idx]),
            Err(idx) => {
                error!(
                    "File: {} should be at {} but no bucket can satisfy that request",
                    id,
                    idx
                );
                None
            }
        }
    }
}

#[test]
fn test_uuid() {
    // .41ms on avg to generate a v4 uuid
    let start = Instant::now();
    let u = Uuid::new_v4();
    let elapsed = start.elapsed();
    println!("uuid generate took {} nanosecs", elapsed.subsec_nanos());
}

#[test]
fn test_dht() {
    let mut dht = Dht::new(None);
    dht.add_node(IpAddr::from_str("192.168.1.1").unwrap(), 1);
    println!("dht: {:#?}", dht);
    dht.add_node(IpAddr::from_str("192.168.1.3").unwrap(), 1);
    println!("dht: {:#?}", dht);
    dht.add_node(IpAddr::from_str("192.168.1.2").unwrap(), 1);
    println!("dht: {:#?}", dht);
    let bucket = dht.locate(&Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap());
    println!("{:#?}", bucket);
    assert_eq!(
        bucket,
        Some(Bucket {
            ip: IpAddr::from_str("192.168.1.1").unwrap(),
            start_range: 0, 
            end_range: 6148914691236517205,
            weight: 1
        })
    );
    dht.remove_node(IpAddr::from_str("192.168.1.3").unwrap());
    println!("dht: {:#?}", dht);
}
