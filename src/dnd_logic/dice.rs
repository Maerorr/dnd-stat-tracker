pub struct Dice {
    pub sides: i32,
    pub count: i32,
}

impl Dice {
    pub fn new(sides: i32, count: i32) -> Self {
        Self { sides, count }
    }

    pub fn max_roll(&self) -> i32 {
        self.sides * self.count
    }
}