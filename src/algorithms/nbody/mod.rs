mod nbody;

use {crate::BenchableExt, nbody::NBodyBenchmark, pinscher::Benchable, std::thread};

// Because benchmarks run iteratively, use smaller constants by default:
const BENCH_BODIES: usize = 1000;
const BENCH_TICKS: usize = 10;

pub struct NBodySeq {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodySeq {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 1;

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

impl BenchableExt for NBodySeq {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

pub struct NBodyParIter {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodyParIter {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 51;

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

impl BenchableExt for NBodyParIter {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

pub struct NBodyParReduce {
    nbody_benchmark: Option<NBodyBenchmark>,
}

impl NBodyParReduce {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 26;

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

impl BenchableExt for NBodyParReduce {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}
