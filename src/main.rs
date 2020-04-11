use std::env;
use std::io;

use logwatch::Watcher;

//-- Choose an implementation --
use logwatch::PollWatcher as LogWatcher;
//use logwatch::TailWatcher as LogWatcher;

fn main() -> Result<(), io::Error> {
    let fpath = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "usage: logwatch /path/to/log"))?;
    let mut watcher = LogWatcher::new(fpath);

    let callback = |line: String| {
        println!("{}", line);
    };
    watcher.register(callback);
    watcher.watch();
    Ok(())
}
