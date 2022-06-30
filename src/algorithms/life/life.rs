use {
    super::Board,
    crate::{Benchable, BenchableExt},
    std::thread,
};

pub struct LifeSeq {
    board: Option<Board>,
}

impl LifeSeq {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 32;

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
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

pub struct LifeParIter {
    board: Option<Board>,
}

impl LifeParIter {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 12;

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
        diam::svg("graph.svg", || {
            let board = self.board.take().unwrap();
            super::parallel_generations(board, 100)
        })
        .expect("Failed to generate svg");
    }
}

impl BenchableExt for LifeParIter {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

pub struct LifeParBridge {
    board: Option<Board>,
}

impl LifeParBridge {
    // LifeParBridge performs badly when executed on multiple cores,
    // since the speedup decreases when the number of cores increase.
    // For this reason, I've decided to limit the number of threads
    // to 8.
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 8;

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
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}
