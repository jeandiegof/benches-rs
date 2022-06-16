use {
    super::{inputs, search::can_cross_par},
    crate::Benchable,
};

pub struct FrogJump {
    stones: Vec<i32>,
}

impl FrogJump {
    pub fn new() -> Self {
        let stones = Self::input();
        Self { stones }
    }

    fn input() -> Vec<i32> {
        inputs::trap(2000, 8)
    }
}

impl Benchable for FrogJump {
    fn name(&self) -> &'static str {
        "FrogJump"
    }

    fn setup(&mut self) {
        self.stones = Self::input()
    }

    fn execute(&mut self) {
        can_cross_par::<scc::HashSet<(usize, i32)>>(&self.stones);
    }
}
