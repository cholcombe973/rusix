
/*
    Hierarchical cluster map: devices in buckets in a tree where devices are always leaves
    Input to algorithm: x = a file name
    select n items of type t
    loop over input i

            root
  host foo   |   host bar
     |               |
  disk.1 disk.2  disk.1 disk.2
*/

extern crate petgraph;

use std::path::Path;

use self::petgraph::{Directed, graphmap::GraphMap};

#[derive(Debug)]
pub struct ClusterMap<'a> {
    tree: GraphMap<Bucket<'a>,f64, Directed>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bucket<'a> {
    pub name: &'a str,
    pub bucket: BucketType<'a>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BucketType<'a> {
    Datacenter,
    Disk(Disk<'a>),
    Host(Host<'a>) ,
    Rack,
    Root,
    Row,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Class {
    Hdd,
    Ssd,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Host<'a> {
    pub id: u64,
    pub name: &'a str,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Disk<'a> {
    pub path: &'a Path,
    pub class: Class,
}

impl<'a> ClusterMap<'a> {
    pub fn new() -> Self {
        let mut map = GraphMap::new();
        // ClusterMap always starts at Root
        map.add_node(Bucket { name: "root", bucket: BucketType::Root});

        ClusterMap {
            tree: map,
        }
    }
    // select n items of type t
    pub fn locate(&self, n: u8, t: BucketType) -> Vec<BucketType>{
        //
        vec![]
    }
}
