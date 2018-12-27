use crate::lib::buffer::Buffer;
use crate::lib::step::Step;
use std::collections::HashMap;

pub struct Hash {

}

impl Hash {
    pub fn new() -> Hash {
        Hash {}
    }
}

impl Step for Hash {
    fn execute(&self, buffer: &Buffer) -> std::io::Result<(Buffer)> {
        println!("Executing Hash: with {:?} and {:?}", buffer.content, buffer.meta);

        let output = Buffer{
            content: buffer.content.to_owned(),
            meta: HashMap::new()
        };

        Ok(output)
    }
}
