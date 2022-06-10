use {
    super::{inputs, search::can_cross_par},
    crate::Benchable,
};

pub struct FrogJump {
    stones: Vec<i32>,
}

impl FrogJump {
    pub fn new() -> Self {
        let stones = inputs::trap(2000, 8);
        Self { stones }
    }
}

impl Benchable for FrogJump {
    fn name(&self) -> &'static str {
        "FrogJump"
    }

    fn execute(&mut self) {
        can_cross_par::<scc::HashSet<(usize, i32)>>(&self.stones);
    }
}
