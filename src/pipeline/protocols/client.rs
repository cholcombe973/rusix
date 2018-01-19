extern crate zmq;

pub struct Client;

// Client sends an RPC request to one or more servers
impl PipelinePlugin for Client {
    fn name() -> &str {"client"}
    fn init(&self, options: &HashMap<String,Value>, subvolumes: Vec<String>){
        
    }
    fn process(&self, name: &str, data: &mut [u8]) -> Result<(&str, &mut [u8]), String>{
        let context = zmq::Context::new();
        let client = context.socket(zmq::DEALER).unwrap();
        let mut rng = thread_rng();
        let identity = format!("{:04X}-{:04X}", rng.gen::<u16>(), rng.gen::<u16>());
        client
        .set_identity(identity.as_bytes())
        .expect("failed setting client id");
        client
        .connect("tcp://localhost:5570")
        .expect("failed connecting client");
        let mut request_nbr = 0;
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
       Err("Not implemented".into()) 
    }

    fn stop(&self){

    }
}
