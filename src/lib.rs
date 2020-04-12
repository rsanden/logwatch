pub mod pollwatch;
pub mod tailwatch;

pub use pollwatch::PollWatcher;
pub use tailwatch::TailWatcher;

use std::time::Duration;

pub trait Watcher<'a> {
    fn new(filename: &str, period: Duration) -> Self;
    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>);
    fn watch(&mut self);
}
