use crate::lib::buffer::Buffer;
use crate::lib::step::Step;
use std::collections::HashMap;

pub struct Reverse {

}

impl Reverse {
    pub fn new() -> Reverse {
        Reverse {}
    }
}

impl Step for Reverse {
    fn execute(&self, buffer: &Buffer) -> std::io::Result<(Buffer)> {
        println!("Executing Reverse: with {:?} and {:?}", buffer.content, buffer.meta);

        let reversed = buffer.content.chars().rev().collect::<String>();

        let output = Buffer{
            content: reversed.to_owned(),
            meta: HashMap::new()
        };

        Ok(output)
    }
}
