use super::PipelinePlugin;
use super::Value;

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
