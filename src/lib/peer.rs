// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::IpAddr;

use lib::config::Config;

/// Add a peer from the cluster
pub fn add_peer(c: &mut Config, addr: &IpAddr, port: u16) -> Result<(), String> {
    // Add peer to peer list
    Ok(())
}

/// Remove a peer from the cluster
pub fn remove_peer(c: &mut Config, addr: &IpAddr, port: u16) -> Result<(), String> {
    // Add peer to peer list
    Ok(())
}
