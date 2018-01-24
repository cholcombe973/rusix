extern crate api;
extern crate futures;
extern crate futures_cpupool;
extern crate zmq;

use std::collections::HashMap;
use std::path::PathBuf;

use self::api::service::*;
use self::futures::Future;
use self::futures_cpupool::CpuPool;
use lib::config::Peer;
use super::super::Value;

pub struct Client {
    pub peer_name: String,
    pool: CpuPool,
}

// Client sends an RPC request to one or more servers
impl Client {
    fn new(&self, name: &str, options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Client{
        let pool = CpuPool::new_num_cpus();
        Client {
            peer_name: name.to_string(), 
            pool: pool,
        }
    }

    // The FOP should be processed before being sent by the client
    fn process_fop(&self, layout: Vec<(Peer, PathBuf)>, io_type: &Fop, data: &mut FileOperation) -> Result<(), String> {
        // Client is the end of the pipeline.
        // send the Fop over to the server(s)
        let context = zmq::Context::new();
        let client = context.socket(zmq::DEALER).unwrap();
        client
            .set_identity(self.peer_name.as_bytes())
            .expect("failed setting client id");

        for entry in layout {
            client
                .connect(&format!("tcp://{}:{}", entry.0.ip, entry.0.port))
                .expect("failed connecting client");

            let request = format!("request #{}", request_nbr);
            client
                .send(&request, 0)
                .expect("client failed sending request");
        }

        loop {
            for _ in 0..100 {
                if client.poll(zmq::POLLIN, 10).expect("client failed polling") > 0 {
                    let msg = client
                        .recv_multipart(0)
                        .expect("client failed receivng response");
                    println!("{}", str::from_utf8(&msg[msg.len() - 1]).unwrap());
                }
            }
            request_nbr = request_nbr + 1;
            let request = format!("request #{}", request_nbr);
            client
                .send(&request, 0)
                .expect("client failed sending request");
        }
        // Packet sent
        //Ok(())
    }

    fn stop(&self) {}
}
