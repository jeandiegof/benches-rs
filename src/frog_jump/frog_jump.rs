use {
    super::{inputs, search::can_cross_par},
    crate::Benchable,
};

pub struct FrogJump {
    stones: Option<Vec<i32>>,
}

impl FrogJump {
    pub fn new() -> Self {
        let stones = None;
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
        self.stones.replace(Self::input());
    }

    fn execute(&mut self) {
        let stones = self.stones.take().unwrap();
        can_cross_par::<scc::HashSet<(usize, i32)>>(&stones);
    }
}
