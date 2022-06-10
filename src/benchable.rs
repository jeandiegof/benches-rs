pub trait Benchable {
    fn name(&self) -> &'static str;
    fn execute(&mut self);
}
