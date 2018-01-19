extern crate zmq;

pub struct Server;

// Server receives an RPC request and responds
impl PipelinePlugin for Server {
    fn name() -> &str {"client"}
    fn init(&self, options: &HashMap<String,Value>, subvolumes: Vec<String>){
        
    }
    fn process(&self, name: &str, data: &mut [u8]) -> Result<(&str, &mut [u8]), String>{
        let context = zmq::Context::new();
        let frontend = context.socket(zmq::ROUTER).unwrap();
        frontend.bind("tcp://*:5570")
            .expect("server failed binding frontend");
        let backend = context.socket(zmq::DEALER).unwrap();
        backend.bind("inproc://backend")
            .expect("server failed binding backend");
        for _ in 0..5 {
            let ctx = context.clone();
            thread::spawn(move || server_worker(&ctx));
        }
       zmq::proxy(&frontend, &backend).expect("server failed proxying");
       Err("Not implemented".into()) 
    }

    fn stop(&self){

    }
}
