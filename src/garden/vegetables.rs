use super::tree::Willow;

#[derive(Debug)]
pub struct Asparagus {}

impl Asparagus {
    pub fn new() -> Self {
        Willow::new();
        Self {}
    }
}
