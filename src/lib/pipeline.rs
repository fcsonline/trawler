use std::fmt;

use crate::lib::buffer::Buffer;
use crate::lib::catalog::Catalog;
use std::collections::HashMap;

pub struct Pipeline {
    pub steps: Vec<String>
}

impl Pipeline {
    pub fn execute(&self, buffer: Buffer) {
        println!("Loading catalog of steps");

        let catalog = Catalog::new();

        let mut output = Buffer{
            content: buffer.content.to_owned(),
            meta: HashMap::new()
        };

        for name in &self.steps {
            let step = catalog.steps.get(name).expect("Unknown step");

            output = step.execute(&output).expect("Step failed");
        }
    }
}

impl fmt::Debug for Pipeline {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.steps[..].fmt(formatter)
    }
}
