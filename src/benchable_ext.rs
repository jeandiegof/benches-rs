use pinscher::Benchable;

pub trait BenchableExt: Benchable + Send {
    fn execution_threads(&self) -> usize;
}

impl<T: BenchableExt + ?Sized> BenchableExt for Box<T> {
    fn execution_threads(&self) -> usize {
        (**self).execution_threads()
    }
}
