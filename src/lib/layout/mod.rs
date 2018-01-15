extern crate rendezvous_hash; 

use std::iter::Iterator;
use self::rendezvous_hash::Node;

pub mod disperse;
pub mod distribute;
pub mod replicate;

pub trait Layout {
    /// A layout entry
    type E: Node+Iterator;
}
