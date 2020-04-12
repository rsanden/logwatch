use std::env;
use std::io;

use logwatch::Watcher;

//-- Choose an implementation --
use logwatch::PollWatcher as LogWatcher;
//use logwatch::TailWatcher as LogWatcher;

fn factory<'a>(count: &'a mut usize) -> Box<dyn FnMut(String) + 'a> {
    let callback = move |line: String| {
        *count += 1;
        println!("{}", line);
    };
    Box::new(callback)
}

fn main() -> Result<(), io::Error> {
    let fpath = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "usage: logwatch /path/to/log"))?;
    let mut count = 0;
    let mut watcher = LogWatcher::new(&fpath, 2000); // polling period = 2000 milliseconds
    let callback = factory(&mut count);
    watcher.register(callback);
    watcher.watch();
    Ok(())
}
