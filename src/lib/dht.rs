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
extern crate byteorder;
extern crate digest;
extern crate meowhash;

use std::cmp::Ordering;
use std::io::Cursor;
use std::net::IpAddr;
use std::path::Path;
use std::time::Instant;

use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use self::digest::Digest;
use self::meowhash::MeowHasher;

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
            let segment_length = u64::max_value() / (self.buckets.len() as u64);

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

    /// Find the node that should contain the hash
    pub fn locate_hash(&self, p: u128, m: u64, n: u64) -> Option<Bucket> {
        let start = Instant::now();
        if self.buckets.len() == 0 {
            return None;
        }
        // Binary search through buckets to find
        // the one containing the correct hash range
        let mut h = MeowHasher::with_seed(200);
        // M+N bytes
        let mut tmp = vec![];
        tmp.write_u128::<LittleEndian>(p).unwrap();
        tmp.write_u64::<LittleEndian>(m).unwrap();
        tmp.write_u64::<LittleEndian>(n).unwrap();
        h.input(&tmp);

        let hash = h.result();
        debug!("hash: {:?}", hash);
        let mut rdr = Cursor::new(hash);
        // Slice off u64 from hash to locate path
        let hash = rdr.read_u64::<LittleEndian>().unwrap();
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
                    p, idx
                );
                None
            }
        }
    }

    /// Find the node that should contain the path
    pub fn locate_path(&self, p: &Path, m: u64, n: u64) -> Option<Bucket> {
        let start = Instant::now();
        if self.buckets.len() == 0 {
            return None;
        }
        // Binary search through buckets to find
        // the one containing the correct hash range
        let mut h = MeowHasher::with_seed(200);
        // M+N bytes
        let mut m_n = vec![];
        m_n.write_u64::<LittleEndian>(m).unwrap();
        m_n.write_u64::<LittleEndian>(n).unwrap();

        h.input(p.to_string_lossy().as_bytes());
        h.input(&m_n);
        let hash = h.result();
        debug!("hash: {:?}", hash);
        let mut rdr = Cursor::new(hash);
        // Slice off u64 from hash to locate path
        let hash = rdr.read_u64::<LittleEndian>().unwrap();
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
                    p.display(),
                    idx
                );
                None
            }
        }
    }
}

#[test]
fn test_dht() {
    use std::str::FromStr;
    let mut dht = Dht::new(None);
    dht.add_node(IpAddr::from_str("192.168.1.1").unwrap(), 1);
    println!("dht: {:#?}", dht);
    dht.add_node(IpAddr::from_str("192.168.1.3").unwrap(), 1);
    println!("dht: {:#?}", dht);
    dht.add_node(IpAddr::from_str("192.168.1.2").unwrap(), 1);
    println!("dht: {:#?}", dht);
    let bucket = dht.locate_path(&Path::new("/tmp/foo/bar/file.txt"), 4, 2);
    println!("{:#?}", bucket);
    assert_eq!(
        bucket,
        Some(Bucket {
            ip: IpAddr::from_str("192.168.1.2").unwrap(),
            start_range: 12297829382473034411,
            end_range: 18446744073709551615,
            weight: 1
        })
    );
    dht.remove_node(IpAddr::from_str("192.168.1.3").unwrap());
    println!("dht: {:#?}", dht);
}
