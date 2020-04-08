use std::env;
use std::io;

use logwatch::Watcher;

fn main() -> Result<(), io::Error> {
    let fpath = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "usage: logwatch /path/to/log"))?;
    let mut watcher = Watcher::new(fpath);

    watcher.register(&|line: String| {
        println!("{}", line);
    });

    watcher.watch();

    Ok(())
}
