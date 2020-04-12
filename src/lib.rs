use std::time::Duration;
pub const PERIOD: Duration = Duration::from_millis(2000);

pub trait Watcher<'a> {
    fn new(filename: String) -> Self;
    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>);
    fn watch(&mut self);
}

pub mod pollwatch;
pub mod tailwatch;

pub use pollwatch::PollWatcher;
pub use tailwatch::TailWatcher;
