// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate contains librusix for other languages to use as bindings.
//! Everything is exported as #repr(c)
extern crate rusix;

#[no_mangle]
pub extern "C" fn mkdir() -> i32 {

    0
}
