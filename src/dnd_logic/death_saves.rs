pub struct DeathSaves {
    pub successes: i32,
    pub failures: i32,
}

impl Default for DeathSaves {
    fn default() -> Self {
        Self { successes: 0, failures: 0 }
    }
}