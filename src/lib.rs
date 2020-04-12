pub mod pollwatch;
pub mod tailwatch;

pub use pollwatch::PollWatcher;
pub use tailwatch::TailWatcher;

pub trait Watcher<'a> {
    fn new(filename: &str, period_milliseconds: u64) -> Self;
    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>);
    fn watch(&mut self);
}
