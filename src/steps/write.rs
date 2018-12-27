use crate::lib::buffer::Buffer;
use crate::lib::step::Step;
use std::collections::HashMap;

use std::fs;

pub struct Write {

}

impl Write {
    pub fn new() -> Write {
        Write {}
    }
}

impl Step for Write {
    fn execute(&self, buffer: &Buffer) -> std::io::Result<(Buffer)> {
        println!("Executing Write: with {:?} and {:?}", buffer.content, buffer.meta);

        fs::write("/tmp/foo", &buffer.content).expect("Unable to write file");


        let output = Buffer{
            content: buffer.content.to_owned(),
            meta: HashMap::new()
        };

        Ok(output)
    }
}
