use crate::lib::buffer::Buffer;
use crate::lib::step::Step;
use std::collections::HashMap;

pub struct Echo {

}

impl Echo {
    pub fn new() -> Echo {
        Echo {}
    }
}

impl Step for Echo {
    fn execute(&self, buffer: &Buffer) -> std::io::Result<(Buffer)> {
        println!("Executing Echo: with {:?} and {:?}", buffer.content, buffer.meta);

        let output = Buffer{
            content: buffer.content.to_owned(),
            meta: HashMap::new()
        };

        Ok(output)
    }
}
