use std::path::PathBuf;
use super::Layout;
use super::super::config::Peer;

pub struct Distribute;

impl Layout for Distribute {
    type E = (Peer, PathBuf);
}
