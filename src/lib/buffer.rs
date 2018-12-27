use std::fmt;
use std::collections::HashMap;

pub struct Buffer {
    pub content: String,
    pub meta: HashMap<String, String>
}

impl fmt::Debug for Buffer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(formatter)
    }
}
