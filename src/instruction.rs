pub struct Instruction(pub Direction, pub i32);

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Direction {
    pub fn from(input: &str) -> Self {
        match input.chars().next().unwrap() {
            'L' => Self::LEFT,
            'R' => Self::RIGHT,
            'U' => Self::UP,
            'D' => Self::DOWN,
            _ => panic!("Unknown char"),
        }
    }
}
