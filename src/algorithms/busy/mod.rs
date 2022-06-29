use {
    crate::{Benchable, BenchableExt},
    std::thread,
};

pub struct Busy;

impl Busy {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 32;

    pub fn new() -> Self {
        Self
    }
}

impl Benchable for Busy {
    fn name(&self) -> &'static str {
        "Busy"
    }

    fn execute(&mut self) {
        for _ in 0..1_000_000_000 {
            unsafe { std::arch::asm!("nop") }
        }
    }
}

impl BenchableExt for Busy {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}
