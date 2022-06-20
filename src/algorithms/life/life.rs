use {
    super::Board,
    crate::{Benchable, BenchableExt},
};

pub struct LifeSeq {
    board: Option<Board>,
}

impl LifeSeq {
    pub fn new() -> Self {
        let board = None;

        Self { board }
    }
}

impl Benchable for LifeSeq {
    fn name(&self) -> &'static str {
        "Life - sequential generation"
    }

    fn setup(&mut self) {
        self.board.replace(Board::new(200, 200).random());
    }

    fn execute(&mut self) {
        let board = self.board.take().unwrap();
        super::generations(board, 100);
    }
}

impl BenchableExt for LifeSeq {
    fn execution_threads(&self) -> usize {
        1
    }
}

pub struct LifeParIter {
    board: Option<Board>,
}

impl LifeParIter {
    pub fn new() -> Self {
        let board = None;

        Self { board }
    }
}

impl Benchable for LifeParIter {
    fn name(&self) -> &'static str {
        "Life - parallel iterators generation"
    }

    fn setup(&mut self) {
        self.board.replace(Board::new(200, 200).random());
    }

    fn execute(&mut self) {
        let board = self.board.take().unwrap();
        super::parallel_generations(board, 100)
    }
}

impl BenchableExt for LifeParIter {
    fn execution_threads(&self) -> usize {
        15
    }
}

pub struct LifeParBridge {
    board: Option<Board>,
}

impl LifeParBridge {
    pub fn new() -> Self {
        let board = None;

        Self { board }
    }
}

impl Benchable for LifeParBridge {
    fn name(&self) -> &'static str {
        "Life - parallel bridge generation"
    }

    fn setup(&mut self) {
        self.board.replace(Board::new(200, 200).random());
    }

    fn execute(&mut self) {
        let board = self.board.take().unwrap();
        super::par_bridge_generations(board, 100)
    }
}

impl BenchableExt for LifeParBridge {
    // LifeParBridge performs badly when executed on multiple cores,
    // since the speedup decreases when the number of cores increase.
    // For this reason, I've decided to limit the number of threads
    // to 8.
    fn execution_threads(&self) -> usize {
        8
    }
}
