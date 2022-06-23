mod algorithms;
mod app_args;
mod bench_record;
mod benchable_ext;
pub use benchable_ext::BenchableExt;

use {
    algorithms::{
        FrogJump, LifeParBridge, LifeParIter, LifeSeq, MergeSort, NBodyParIter, NBodyParReduce,
        NBodySeq, QuickSort, Tsp,
    },
    app_args::AppArgs,
    bench_record::BenchRecord,
    csv::Writer,
    pinscher::{AllBenchers, BenchSuite, Benchable},
    rayon::ThreadPoolBuilder,
};

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();

    for i in 1..=args.runs() {
        let mut algorithms = algorithms();

        for algorithm in &mut algorithms {
            println!("Running {} {}/{}", algorithm.name(), i, args.runs());
            let bench_results = bench(algorithm);
            let time_bencher = bench_results.time_bencher();
            println!("{}", time_bencher.real_time().unwrap().as_micros());
        }
    }
}

fn algorithms() -> Vec<Box<dyn BenchableExt>> {
    vec![Box::new(LifeSeq::new())]
}

fn bench<T>(algorithm: &mut T) -> AllBenchers
where
    T: BenchableExt,
{
    let threads = 4;
    let mut all_benchers = AllBenchers::new().unwrap();
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    pool.install(|| BenchSuite::bench(algorithm, &mut all_benchers).unwrap());

    all_benchers
}

pub fn seeded_rng() -> rand_xorshift::XorShiftRng {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
