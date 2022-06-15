mod merge_sort;
use merge_sort::MergeSort;

mod bench_record;
use bench_record::BenchRecord;

mod frog_jump;
use frog_jump::FrogJump;

mod life;
use life::{LifeParBridge, LifeParIter, LifeSeq};

mod nbody;
use nbody::{NBodyParIter, NBodyParReduce, NBodySeq};

mod quicksort;
use quicksort::QuickSort;

mod tsp;
use tsp::Tsp;

mod app_args;
use app_args::AppArgs;

use {
    csv::Writer,
    pinscher::{BenchSuite, Benchable, CpuTimeBencher, EnergyBencher},
};

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();

    for i in 1..=args.runs() {
        let mut algorithms = algorithms();
        for algorithm in &mut algorithms {
            println!("Running {} {}/{}", algorithm.name(), i, args.runs());
            let (cpu_time, energy) = bench(algorithm);
            save_results(&mut csv_writer, algorithm.name(), cpu_time, energy);
        }
    }
}

fn algorithms() -> Vec<Box<dyn Benchable>> {
    vec![
        Box::new(MergeSort::new(8)),
        Box::new(FrogJump::new()),
        Box::new(LifeSeq::new()),
        Box::new(LifeParIter::new()),
        Box::new(LifeParBridge::new()),
        Box::new(NBodyParIter::new()),
        Box::new(NBodyParReduce::new()),
        Box::new(NBodySeq::new()),
        Box::new(QuickSort::new()),
        Box::new(Tsp::new()),
    ]
}

fn bench<T>(algorithm: &mut T) -> (CpuTimeBencher, EnergyBencher)
where
    T: Benchable,
{
    let mut cpu_time_bencher = CpuTimeBencher::new();
    BenchSuite::bench(algorithm, &mut cpu_time_bencher).unwrap();

    let mut energy_bencher = EnergyBencher::new().unwrap();
    BenchSuite::bench(algorithm, &mut energy_bencher).unwrap();

    (cpu_time_bencher, energy_bencher)
}

fn save_results<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    cpu_time: CpuTimeBencher,
    energy: EnergyBencher,
) {
    let name = name.to_string();
    let hostname = sys_info::hostname().unwrap_or("unknown".to_string());
    let record = BenchRecord::new(name, hostname, cpu_time, energy);
    writer.serialize(record).unwrap();
}

pub fn seeded_rng() -> rand_xorshift::XorShiftRng {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
