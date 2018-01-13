/*
All operations are driven by clients, which are all equal. 
There are no special nodes with special knowledge of where files are or should be.
Directories exist on all subvolumes (bricks or lower-level aggregations of bricks); 
files exist on only one.
Files are assigned to subvolumes based on consistent hashing, 
and even more specifically a form of consistent hashing exemplified by Amazon's Dynamo.
*/
extern crate hashring;

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use self::hashring::HashRing;
use super::PipelinePlugin;
use super::Value;

#[derive(Debug, Copy, Clone)]
struct VNode {
    id: usize,
    addr: SocketAddr,
}

impl VNode {
    fn new(ip: &str, port: u16, id: usize) -> Self {
        let addr = SocketAddr::new(IpAddr::from_str(&ip).unwrap(), port);
        VNode { id: id, addr: addr }
    }
}

impl ToString for VNode {
    fn to_string(&self) -> String {
        format!("{}|{}", self.addr, self.id)
    }
}

impl PartialEq for VNode {
    fn eq(&self, other: &VNode) -> bool {
        self.id == other.id && self.addr == other.addr
    }
}

impl PipelinePlugin for VNode {
    fn name(&self) -> &str {
        "distribute"
    }
    fn init(&self, options: HashMap<String, Value>, subvolumes: Vec<String>) {}

    fn process(&self, name: &str, data: &mut [u8]) -> Result<(&str, &mut [u8]), String> {
        let mut ring: HashRing<VNode, &str> = HashRing::new();

        let mut nodes = vec![];
        nodes.push(VNode::new("127.0.0.1", 1024, 1));
        nodes.push(VNode::new("127.0.0.1", 1024, 2));

        nodes.push(VNode::new("127.0.0.2", 1024, 1));
        nodes.push(VNode::new("127.0.0.2", 1024, 2));

        nodes.push(VNode::new("127.0.0.3", 1024, 1));
        nodes.push(VNode::new("127.0.0.3", 1024, 2));

        for node in nodes {
            ring.add(node);
        }

        ring.remove(VNode::new("127.0.0.1", 1024, 1));

        Err("Foo".to_string())
    }
    fn stop(&self) {}
}
