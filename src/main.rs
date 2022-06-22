#[allow(dead_code)]
mod algorithms;
mod app_args;
mod bench_record;
mod benchable_ext;
pub use benchable_ext::BenchableExt;

use {
    algorithms::{LifeSeq, NBodySeq},
    app_args::AppArgs,
    bench_record::BenchRecord,
    csv::Writer,
    pinscher::{AllBenchers, BenchSuite, Benchable},
    rayon::ThreadPoolBuilder,
    std::sync::{Arc, Mutex},
};

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();

    for i in 1..=args.runs() {
        let mut algorithms = algorithms();

        for algorithm in &mut algorithms {
            println!("Running {} {}/{}", algorithm.name(), i, args.runs());
            let bench_results = bench(algorithm);
            save_results(&mut csv_writer, algorithm, bench_results);
        }
    }
}

fn algorithms() -> Vec<Box<dyn BenchableExt>> {
    vec![Box::new(LifeSeq::new()), Box::new(NBodySeq::new())]
}

fn bench<T>(algorithm: &mut T) -> AllBenchers
where
    T: BenchableExt,
{
    let mut all_benchers = AllBenchers::new().unwrap();
    let threads = std::env::var("RAYON_NUM_THREADS").unwrap();

    let core_ids = Arc::new(Mutex::new(core_affinity::get_core_ids().unwrap()));

    let pool = ThreadPoolBuilder::new()
        .num_threads(threads.parse().unwrap())
        .spawn_handler(|thread| {
            let core_ids_cloned = core_ids.clone();

            std::thread::spawn(move || {
                let core_id = core_ids_cloned.lock().unwrap().pop().unwrap();
                core_affinity::set_for_current(core_id);

                thread.run();
            });

            Ok(())
        })
        .build()
        .unwrap();

    pool.install(|| ());
    BenchSuite::bench(algorithm, &mut all_benchers).unwrap();

    all_benchers
}

fn save_results<W, T>(writer: &mut Writer<W>, algorithm: &T, all_benchers: AllBenchers)
where
    W: std::io::Write,
    T: BenchableExt,
{
    let name = algorithm.name().to_string();
    let hostname = sys_info::hostname().unwrap_or("unknown".to_string());
    let threads = algorithm.execution_threads();

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
