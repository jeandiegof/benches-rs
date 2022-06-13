use rand::distributions::Standard;
use rand::Rng;

use crate::Benchable;

const BENCH_SIZE: usize = 250_000_000 / 512;

pub struct QuickSort;

impl QuickSort {
    pub fn new() -> Self {
        Self
    }
}

impl Benchable for QuickSort {
    fn name(&self) -> &'static str {
        "QuickSort"
    }

    fn execute(&mut self) {
        // TODO: the creation of the data to be sorted shouldn't be
        // inside the `execute` function. If it is, the time taken
        // to create the vector will be taken into account during the
        // benchmarking.
        // For some reason, when I create the vector inside the `new()`
        // and try to store in a field of the QuickSort struct, I get
        // a stack overflow which I don't understand why, since the
        // Vector should be allocated in the heap and we are only
        // passing the reference around

        let mut data = default_vec(BENCH_SIZE);
        quick_sort::<Parallel, u32>(&mut data[..]);
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
        return quick_sort::<Sequential, T>(v);
    }

    let mid = partition(v);
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
