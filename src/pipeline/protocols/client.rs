extern crate api;
extern crate futures;
extern crate futures_cpupool;
extern crate zmq;

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;

use self::api::service::*;
use self::futures::Future;
use self::futures_cpupool::CpuPool;
use super::super::Value;
use lib::config::Peer;

pub struct Client {
    pub peer_name: String,
    pool: CpuPool,
}

// Client sends an RPC request to one or more servers
impl Client {
    fn new(&self, name: &str, options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Client {
        let pool = CpuPool::new_num_cpus();
        Client {
            peer_name: name.to_string(),
            pool: pool,
        }
    }

    // The FOP should be processed before being sent by the client
    fn process_fop(
        &self,
        layout: Vec<(Peer, PathBuf)>,
        io_type: &Fop,
        data: &mut FileOperation,
    ) -> Result<(), String> {
        // Client is the end of the pipeline.
        // send the Fop over to the server(s)
        let context = zmq::Context::new();

        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for entry in layout {
            // TODO: Change over to cpupool?
            /*
            let t = thread::spawn(|| {
                let client = context.socket(zmq::REQ).expect("socket creation failed");
                client
                    .set_identity(self.peer_name.as_bytes())
                    .expect("failed setting client id");
                client
                    .connect(&format!("tcp://{}:{}", entry.0.ip, entry.0.port))
                    .expect("failed connecting client");
                client.send(&vec![], 0).map_err(|e| e.to_string()).expect("request failed");
            });
            handles.push(t);
            */
        }
        // Wait for completion
        for h in handles {
            h.join();
        }
        // Packet sent
        Ok(())
    }

    fn stop(&self) {}
}
