mod nbody;

use crate::Benchable;
use nbody::NBodyBenchmark;

// Because benchmarks run iteratively, use smaller constants by default:
const BENCH_BODIES: usize = 1000;
const BENCH_TICKS: usize = 10;

pub struct NBodySeq {
    nbody_benchmark: NBodyBenchmark,
}

impl NBodySeq {
    pub fn new() -> Self {
        let mut rng = crate::seeded_rng();
        let nbody_benchmark = NBodyBenchmark::new(BENCH_BODIES, &mut rng);

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodySeq {
    fn name(&self) -> &'static str {
        "NBody sequential"
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.tick_seq();
        }
    }
}

pub struct NBodyParIter {
    nbody_benchmark: NBodyBenchmark,
}

impl NBodyParIter {
    pub fn new() -> Self {
        let mut rng = crate::seeded_rng();
        let nbody_benchmark = NBodyBenchmark::new(BENCH_BODIES, &mut rng);

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodyParIter {
    fn name(&self) -> &'static str {
        "NBody parallel iterator"
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.tick_par();
        }
    }
}

pub struct NBodyParReduce {
    nbody_benchmark: NBodyBenchmark,
}

impl NBodyParReduce {
    pub fn new() -> Self {
        let mut rng = crate::seeded_rng();
        let nbody_benchmark = NBodyBenchmark::new(BENCH_BODIES, &mut rng);

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodyParReduce {
    fn name(&self) -> &'static str {
        "NBody parallel reduce"
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.tick_par_reduce();
        }
    }
}
