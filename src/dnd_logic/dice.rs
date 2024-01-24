use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dice {
    pub sides: i32,
    pub count: i32,
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.count, self.sides)
    }
}

impl Dice {
    pub fn new(sides: i32, count: i32) -> Self {
        Self { sides, count }
    }

    pub fn max_roll(&self) -> i32 {
        self.sides * self.count
    }
}