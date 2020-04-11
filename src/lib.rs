use std::time::Duration;
pub const PERIOD: Duration = Duration::from_millis(2000);

pub trait Watcher<'a> {
    fn new(filename: String) -> Self;
    fn register<F: Fn(String) + 'a>(&mut self, callback: F);
    fn watch(&mut self);
}

pub mod pollwatch;
pub mod tailwatch;
