use pinscher::AllBenchers;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BenchRecord {
    /// name of the algorithm
    name: String,

    /// id of the machine used to perform the experiment
    machine_id: String,

    /// number of threads in the thread pool
    threads: usize,

    /// CPU active time in micro seconds
    cpu_time_us: u128,

    /// Wall clock time
    wall_clock_time_us: u128,

    /// Package energy in micro Joules
    package_energy: u64,

    /// Core energy in micro Joules
    core_energy: u64,
}

impl BenchRecord {
    pub fn new(
        name: String,
        machine_id: String,
        threads: usize,
        all_benchers: AllBenchers,
    ) -> Self {
        let cpu_time_bencher = all_benchers.cpu_time_bencher();
        let time_bencher = all_benchers.time_bencher();
        let energy_bencher = all_benchers.energy_bencher();

        let cpu_time_us = cpu_time_bencher.cpu_time().unwrap().as_micros();
        let wall_clock_time_us = time_bencher.real_time().unwrap().as_micros();
        let package_energy = energy_bencher.package_energy();
        let core_energy = energy_bencher.core_energy();

        Self {
            name,
            machine_id,
            threads,
            cpu_time_us,
            wall_clock_time_us,
            package_energy,
            core_energy,
        }
    }
}
