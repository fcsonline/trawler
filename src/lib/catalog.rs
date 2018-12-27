use std::collections::HashMap;

use crate::lib::step::Step;

use crate::steps::{write::Write, hash::Hash, echo::Echo, reverse::Reverse};

pub struct Catalog {
    pub steps: HashMap<String, Box<Step>>
}

impl Catalog {
    pub fn new() -> Catalog {
        let mut steps: HashMap<String, Box<Step>> = HashMap::new();

        steps.insert("write".to_string(), Box::new(Write::new()));
        steps.insert("hash".to_string(), Box::new(Hash::new()));
        steps.insert("reverse".to_string(), Box::new(Reverse::new()));
        steps.insert("echo".to_string(), Box::new(Echo::new()));

        Catalog {
            steps,
        }
    }
}
