mod benchable;
use benchable::Benchable;

mod merge_sort;
use merge_sort::MergeSort;

mod bench_record;
use bench_record::BenchRecord;

mod frog_jump;
use frog_jump::FrogJump;

mod app_args;
use app_args::AppArgs;

use {
    csv::Writer,
    pinscher::{BenchSuite, CpuTimeBencher, EnergyBencher},
};

fn main() {
    let args = AppArgs::new();
    let mut csv_writer = Writer::from_path(args.output_filename()).unwrap();
    let mut algorithms: Vec<Box<dyn Benchable>> =
        vec![Box::new(MergeSort::new(8)), Box::new(FrogJump::new())];

    for i in 1..=args.runs() {
        for algorithm in &mut algorithms {
            let name = algorithm.name();

            println!("Running {} {}/{}", name, i, args.runs());
            let (cpu_time, energy) = bench(algorithm);
            save_results(&mut csv_writer, name, cpu_time, energy);
        }
    }
}

fn bench(algorithm: &mut Box<dyn Benchable>) -> (CpuTimeBencher, EnergyBencher) {
    let mut cpu_time_bencher = CpuTimeBencher::new();
    BenchSuite::bench(|| algorithm.execute(), &mut cpu_time_bencher).unwrap();

    let mut energy_bencher = EnergyBencher::new().unwrap();
    BenchSuite::bench(|| algorithm.execute(), &mut energy_bencher).unwrap();

    (cpu_time_bencher, energy_bencher)
}

fn save_results<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    cpu_time: CpuTimeBencher,
    energy: EnergyBencher,
) {
    let record = BenchRecord::new(name.to_string(), cpu_time, energy);
    writer.serialize(record).unwrap();
}
