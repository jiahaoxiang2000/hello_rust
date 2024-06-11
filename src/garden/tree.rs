use super::vegetables;

#[derive(Debug)]
pub struct Willow {
    age: u32,
}

impl Willow {
    pub fn new() -> Willow {
        vegetables::Asparagus {};
        Willow { age: 0 }
    }
}

