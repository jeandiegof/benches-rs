use crate::BenchableExt;
use rand::distributions::Standard;
use rand::Rng;

use crate::Benchable;
use std::thread;
use std::time::Instant;

const BENCH_SIZE: usize = 250_000_000 / 512;

pub struct QuickSort {
    data: Option<Vec<u32>>,
    run: usize,
}

impl QuickSort {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 15;

    pub fn new() -> Self {
        let data = None;
        Self { data, run: 0 }
    }
}

impl Benchable for QuickSort {
    fn name(&self) -> &'static str {
        "QuickSort"
    }

    fn setup(&mut self) {
        self.data.replace(default_vec(BENCH_SIZE));
    }

    fn execute(&mut self) {
        if let Some(data) = self.data.as_mut() {
            let filename = &format!("quicksort-{}.json", self.run);
            diam::gantt_json(filename, || {
                let begin = Instant::now();
                quick_sort::<Parallel, u32>(data);
                println!("{}: {}", self.run, begin.elapsed().as_micros());
            })
            .unwrap();
            self.run = self.run + 1;
        } else {
            panic!();
        }
    }

    fn teardown(self: &mut QuickSort) {
        if let Some(data) = self.data.as_mut() {
            assert!(data.windows(2).all(|w| w[0] <= w[1]))
        } else {
            panic!();
        }
    }
}

impl BenchableExt for QuickSort {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

pub trait Joiner {
    fn is_parallel() -> bool;
    fn join<A, RA, B, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send;
}

pub struct Parallel;
impl Joiner for Parallel {
    #[inline]
    fn is_parallel() -> bool {
        true
    }

    #[inline]
    fn join<A, RA, B, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send,
    {
        rayon::join(oper_a, oper_b)
    }
}

struct Sequential;
impl Joiner for Sequential {
    #[inline]
    fn is_parallel() -> bool {
        false
    }

    #[inline]
    fn join<A, RA, B, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send,
    {
        let a = oper_a();
        let b = oper_b();
        (a, b)
    }
}

pub fn quick_sort<J: Joiner, T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if J::is_parallel() && v.len() <= 5 * 1024 {
        let span = tracing::span!(tracing::Level::TRACE, "seq");
        let _guard = span.enter();
        return quick_sort::<Sequential, T>(v);
    }

    let mid = {
        if J::is_parallel() {
            let span = tracing::span!(tracing::Level::TRACE, "part");
            let _guard = span.enter();
            partition(v)
        } else {
            partition(v)
        }
    };
    let (lo, hi) = v.split_at_mut(mid);
    J::join(|| quick_sort::<J, T>(lo), || quick_sort::<J, T>(hi));
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn default_vec(n: usize) -> Vec<u32> {
    let rng = crate::seeded_rng();
    rng.sample_iter(&Standard).take(n).collect()
}
