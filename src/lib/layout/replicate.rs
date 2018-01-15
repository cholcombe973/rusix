use std::path::PathBuf;

use super::Layout;
use super::super::config::Peer;

pub struct Replicate;

impl Layout for Replicate {
    type E = Vec<(Peer, PathBuf)>;
}
