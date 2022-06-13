use {super::Board, crate::Benchable};

pub struct LifeSeq;

impl LifeSeq {
    pub fn new() -> Self {
        Self {}
    }
}

impl Benchable for LifeSeq {
    fn name(&self) -> &'static str {
        "Life - sequential generation"
    }

    fn execute(&mut self) {
        super::generations(Board::new(200, 200).random(), 100);
    }
}

pub struct LifeParIter;

impl LifeParIter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Benchable for LifeParIter {
    fn name(&self) -> &'static str {
        "Life - parallel iterators generation"
    }

    fn execute(&mut self) {
        super::parallel_generations(Board::new(200, 200).random(), 100)
    }
}

pub struct LifeParBridge;

impl LifeParBridge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Benchable for LifeParBridge {
    fn name(&self) -> &'static str {
        "Life - parallel bridge generation"
    }

    fn execute(&mut self) {
        super::par_bridge_generations(Board::new(200, 200).random(), 100)
    }
}
