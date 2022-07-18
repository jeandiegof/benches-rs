mod algorithms;
mod app_args;
mod bench_record;
mod benchable_ext;
pub use benchable_ext::BenchableExt;
use rayon::ThreadPool;

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
    std::{
        sync::{Arc, Mutex},
        thread,
    },
};

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();
    let mut algorithms = algorithms();

    for algorithm in &mut algorithms {
        for i in 1..=args.runs() {
            println!("Running {} {}/{}", algorithm.name(), i, args.runs());
            let pool = build_thread_pool(algorithm);
            let bench_results = bench(algorithm, &pool);
            save_results(&mut csv_writer, algorithm, bench_results);
        }
    }
}

fn algorithms() -> Vec<Box<dyn BenchableExt>> {
    vec![
        // Box::new(MergeSort::new(8)),
        // Box::new(FrogJump::new()),
        // Box::new(LifeSeq::new()),
        // Box::new(LifeParIter::new()),
        // Box::new(LifeParBridge::new()),
        // Box::new(NBodyParIter::new()),
        // Box::new(NBodyParReduce::new()),
        // Box::new(NBodySeq::new()),
        // Box::new(QuickSort::new()),
        Box::new(Tsp::new()),
    ]
}

fn build_thread_pool<T>(algorithm: &mut T) -> ThreadPool
where
    T: BenchableExt,
{
    let threads = std::env::var("RAYON_NUM_THREADS")
        .map(|t| t.parse().unwrap())
        .unwrap_or_else(|_| algorithm.execution_threads());

    let core_ids = Arc::new(Mutex::new(core_affinity::get_core_ids().unwrap()));

    ThreadPoolBuilder::new()
        .num_threads(threads)
        .spawn_handler(|thread| {
            let core_ids_cloned = core_ids.clone();

            thread::spawn(move || {
                let core_id = core_ids_cloned.lock().unwrap().pop().unwrap();
                core_affinity::set_for_current(core_id);

                thread.run();
            });

            Ok(())
        })
        .build()
        .unwrap()
}

fn bench<T>(algorithm: &mut T, pool: &ThreadPool) -> AllBenchers
where
    T: BenchableExt,
{
    let mut all_benchers = AllBenchers::new().unwrap();
    pool.install(|| BenchSuite::bench(algorithm, &mut all_benchers).unwrap());

    all_benchers
}

fn save_results<W, T>(writer: &mut Writer<W>, algorithm: &T, all_benchers: AllBenchers)
where
    W: std::io::Write,
    T: BenchableExt,
{
    let name = algorithm.name().to_string();
    let hostname = sys_info::hostname().unwrap_or_else(|_| "unknown".to_string());
    let threads = std::env::var("RAYON_NUM_THREADS")
        .map(|t| t.parse().unwrap())
        .unwrap_or_else(|_| algorithm.execution_threads());

    let record = BenchRecord::new(name, hostname, threads, all_benchers);
    writer.serialize(record).unwrap();
}

pub fn seeded_rng() -> rand_xorshift::XorShiftRng {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
