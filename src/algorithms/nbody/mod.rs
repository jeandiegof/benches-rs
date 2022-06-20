mod nbody;

use crate::Benchable;
use nbody::NBodyBenchmark;

// Because benchmarks run iteratively, use smaller constants by default:
const BENCH_BODIES: usize = 1000;
const BENCH_TICKS: usize = 10;

pub struct NBodySeq {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodySeq {
    pub fn new() -> Self {
        let nbody_benchmark = None;

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodySeq {
    fn name(&self) -> &'static str {
        "NBody sequential"
    }

    fn setup(&mut self) {
        let mut rng = crate::seeded_rng();
        self.nbody_benchmark
            .replace(NBodyBenchmark::new(BENCH_BODIES, &mut rng));
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.as_mut().unwrap().tick_seq();
        }
    }
}

pub struct NBodyParIter {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodyParIter {
    pub fn new() -> Self {
        let nbody_benchmark = None;

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodyParIter {
    fn name(&self) -> &'static str {
        "NBody parallel iterator"
    }

    fn setup(&mut self) {
        let mut rng = crate::seeded_rng();
        self.nbody_benchmark
            .replace(NBodyBenchmark::new(BENCH_BODIES, &mut rng));
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.as_mut().unwrap().tick_par();
        }
    }
}

pub struct NBodyParReduce {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodyParReduce {
    pub fn new() -> Self {
        let nbody_benchmark = None;

        Self { nbody_benchmark }
    }
}

impl Benchable for NBodyParReduce {
    fn name(&self) -> &'static str {
        "NBody parallel reduce"
    }

    fn setup(&mut self) {
        let mut rng = crate::seeded_rng();
        self.nbody_benchmark
            .replace(NBodyBenchmark::new(BENCH_BODIES, &mut rng));
    }

    fn execute(&mut self) {
        for _ in 0..BENCH_TICKS {
            self.nbody_benchmark.as_mut().unwrap().tick_par_reduce();
        }
    }
}
