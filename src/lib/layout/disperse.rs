// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::hash::Hash;
use std::path::PathBuf;

use super::super::dht;
use super::super::config::Peer;

/// Store files across a set of servers and paths.  Disperse
/// will erasure code a file and distribute it across X Peers
/// and Paths while also storing redundancy pieces of the file
/// on X Peers and Paths
pub struct Disperse {
    entry: Vec<(Peer, PathBuf)>,
    redundancy: Vec<(Peer, PathBuf)>,
}

