#![cfg_attr(feature = "with-bench", feature(test))]

#[cfg(all(test, feature = "with-bench"))]
extern crate test;

extern crate capnp;
extern crate protobuf;

pub mod err;
pub mod service;
pub mod service_capnp;

#[cfg(test)]
mod tests {
    use capnp::{message, serialize, message::ReaderOptions, Word};

    use protobuf::{Message, parse_from_bytes};
    use service::*;
    use service_capnp::stat_request;
    use test::{black_box, Bencher};

    fn create_proto_stat_request() {
        let mut stat = StatRequest::new();
        stat.set_gfid("foo".to_string());

        let encoded = stat.write_to_bytes().unwrap();
    }

    fn read_proto_stat_request(bytes: &Vec<u8>) {
        parse_from_bytes::<StatRequest>(&bytes).unwrap();
    }

    fn read_cap_stat_request(words: &Vec<Word>){
        serialize::read_message_from_words(&words, ReaderOptions::new());
    }

    fn create_cap_stat_request() {
        let mut message = message::Builder::new_default();
        {
            let mut stat_request = message.init_root::<stat_request::Builder>();
            stat_request.set_gfid("foo");
        }

        serialize::write_message_to_words(&message);
    }
    #[bench]
    fn bench_capnp_read(b: &mut Bencher) {
        let mut message = message::Builder::new_default();
        {
            let mut stat_request = message.init_root::<stat_request::Builder>();
            stat_request.set_gfid("foo");
        }

        let words = serialize::write_message_to_words(&message);

        b.iter(|| {
            // Inner closure, the actual test
            for _ in 1..100 {
                black_box(read_cap_stat_request(&words));
            }
        });
    }

    #[bench]
    fn bench_protobuf_read(b: &mut Bencher) {
        let mut stat = StatRequest::new();
        stat.set_gfid("foo".to_string());
        let bytes = stat.write_to_bytes().unwrap();

        b.iter(|| {
            // Inner closure, the actual test
            for _ in 1..100 {
                black_box(read_proto_stat_request(&bytes));
            }
        });
    }

    #[bench]
    fn bench_protobuf(b: &mut Bencher) {
        b.iter(|| {
            // Inner closure, the actual test
            for _ in 1..100 {
                black_box(create_proto_stat_request());
            }
        });
    }

    #[bench]
    fn bench_capnp(b: &mut Bencher) {
        b.iter(|| {
            // Inner closure, the actual test
            for _ in 1..100 {
                black_box(create_cap_stat_request());
            }
        });
    }
}
