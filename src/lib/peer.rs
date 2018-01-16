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
