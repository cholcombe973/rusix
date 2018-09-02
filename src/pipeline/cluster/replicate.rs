// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::super::PipelinePlugin;
use super::super::Value;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Replicate {
    foo: String,
}

impl PipelinePlugin for Replicate {
    fn name(&self) -> &str {
        "replicate"
    }
    fn init(&self, options: HashMap<String, Value>, subvolumes: Vec<String>) {}

    fn process(&self, name: &str, data: &mut [u8]) -> Result<(&str, &mut [u8]), String> {
        Err("Foo".to_string())
    }
    fn stop(&self) {}
}
