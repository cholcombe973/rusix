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

pub mod service_generated;

#[cfg(test)]
mod tests {
    use service_generated::*;
    use test::{black_box, Bencher};

}
