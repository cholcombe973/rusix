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
#[macro_use]
extern crate log;
extern crate rusix;

use rusix::lib::context::Context;
use rusix::lib::dht::Dht;
use rusix::pipeline::protocols::client::Client;

///rusix_new: Create a new 'virtual mount' object.
///This is most likely the very first function you will use. This function
///will create a new context (virtual mount) object in memory.
///RETURN VALUES
///NULL   : Out of memory condition.
///Others : Pointer to the newly created context virtual mount object.
#[no_mangle]
pub extern fn rusix_new(vol_name: &str)  -> *mut Context {
    Box::into_raw(Box::new(Context::new()))
}

#[no_mangle]
pub extern fn mkdir(ptr: *mut Context) -> i32 {
    let ctx = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let dht = Dht::new(None);
    let client = Client::new("client1", None, None);
    let parent_server = match dht.locate_path(&ctx.parent_dir, m, n){
        Some(s) => s,
        None => {
            // Parent server could not be located
            error!("Unable to locate server for {}", ctx.parent_dir.display());
            return -1;
        }
    };

    0
}
