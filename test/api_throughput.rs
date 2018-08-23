#![feature(test)]
extern crate api;
extern crate test;
extern crate protobuf;

#[cfg(test)]
mod tests {
    use api::service::*;
    use api::service_capnp::*;
    use protobuf::Message;
    use test::{black_box, Bencher};

    fn create_proto_message(){
        let mut o = FileOperation::new();
        let mut stat = StatRequest::new();
        stat.set_gfid("foo".to_string());
        o.set_stat_req(stat);

        let encoded = o.write_to_bytes().unwrap();
        let mut m = Message::new();
        let msg = m.merge_from_bytes(&encoded).unwrap();
    }

    fn create_cap_message(){
        //
    }

    #[bench]
    fn bench_protobuf(b: &mut Bencher) {

        b.iter(|| {
            // Inner closure, the actual test
            for i in 1..100 {
                black_box(create_proto_message());
            }
        });
    }

    #[bench]
    fn bench_capnp(b: &mut Bencher) {
        b.iter(|| {
            // Inner closure, the actual test
            for i in 1..100 {
                black_box(create_cap_message());
            }
        });
    }
}
