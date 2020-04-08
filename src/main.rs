use std::env;
use std::io;

use logwatch::Watcher;

//-- Choose an implementation --
use logwatch::pollwatch::LogWatcher;
//use logwatch::tailwatch::LogWatcher;

fn main() -> Result<(), io::Error> {
    let fpath = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "usage: logwatch /path/to/log"))?;
    let mut watcher = LogWatcher::new(fpath);

    watcher.register(&|line: String| {
        println!("{}", line);
    });

    watcher.watch();

    Ok(())
}
