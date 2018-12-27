use crate::lib::buffer::Buffer;

pub trait Step {
    fn execute(&self, buffer: &Buffer) -> std::io::Result<(Buffer)>;
}
