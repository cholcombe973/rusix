// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "with-bench", feature(test))]

#[cfg(all(test, feature = "with-bench"))]
extern crate test;

extern crate flatbuffers;
extern crate protobuf;

pub mod err;
pub mod service;
pub mod service_generated;

#[cfg(test)]
mod tests {
    use protobuf::{parse_from_bytes, Message};
    use service::*;
    use test::{black_box, Bencher};

    fn create_proto_stat_request() {
        let mut stat = StatRequest::new();
        stat.set_gfid("foo".to_string());

        let encoded = stat.write_to_bytes().unwrap();
    }

    fn read_proto_stat_request(bytes: &Vec<u8>) {
        parse_from_bytes::<StatRequest>(&bytes).unwrap();
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

}
