use pinscher::Benchable;

pub trait BenchableExt: Benchable {
    fn execution_threads(&self) -> usize;
}
