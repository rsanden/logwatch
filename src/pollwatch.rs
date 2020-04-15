use crate::Watcher;
use log::warn;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::os::unix::fs::MetadataExt;
use std::thread::sleep;
use std::time::Duration;

pub struct PollWatcher<'a> {
    filename: String,
    inode: u64,
    pos: u64,
    reader: Option<BufReader<File>>,
    initial: bool,
    period: Duration,
    callbacks: Vec<Box<dyn FnMut(String) + 'a>>,
}

impl<'a> PollWatcher<'a> {
    fn seek(&mut self) {
        self.reader
            .as_mut()
            .unwrap()
            .seek(io::SeekFrom::Start(self.pos))
            .unwrap();
    }

    fn reload(&mut self, fp: File, metadata: fs::Metadata) {
        self.inode = metadata.ino();
        if self.initial {
            self.pos = metadata.len();
        } else {
            self.pos = 0;
        }
        self.reader = Some(BufReader::new(fp));
        self.seek();
        self.initial = false;
    }

    fn ready(&mut self) -> Result<(), io::Error> {
        let fp = File::open(self.filename.as_str())?;
        let metadata = fp.metadata()?;
        if self.inode != metadata.ino() {
            self.reload(fp, metadata)
        }
        Ok(())
    }

    fn read_line(&mut self, mut line: &mut String) -> Result<usize, io::Error> {
        line.clear();
        self.reader.as_mut().unwrap().read_line(&mut line)
    }

    fn execute_callbacks(&mut self, line: &str) {
        for callback in &mut self.callbacks {
            callback(line.replace("\n", ""));
        }
    }

    fn process_all_lines(&mut self) {
        let mut line = String::new();
        loop {
            match self.read_line(&mut line) {
                Ok(0) => break,
                Ok(len) => {
                    self.pos += len as u64;
                    self.seek();
                    self.execute_callbacks(&line);
                }
                Err(e) => {
                    warn!("unable to read line: {}", e);
                    sleep(self.period);
                    break;
                }
            }
        }
    }
}

impl<'a> Watcher<'a> for PollWatcher<'a> {
    fn new(filename: &str, period_milliseconds: u64) -> Self {
        PollWatcher {
            filename: filename.to_string(),
            inode: 0,
            pos: 0,
            reader: None,
            initial: true,
            period: Duration::from_millis(period_milliseconds),
            callbacks: vec![],
        }
    }

    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>) {
        self.callbacks.push(callback);
    }

    fn watch(&mut self) {
        loop {
            sleep(self.period);
            if self.ready().is_ok() {
                self.process_all_lines();
            }
        }
    }
}
