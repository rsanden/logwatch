use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::os::unix::fs::MetadataExt;
use std::thread::sleep;
use std::time::Duration;

pub const PERIOD: Duration = Duration::from_millis(2000);

pub struct Watcher<'a> {
    filename: String,
    inode: u64,
    pos: u64,
    reader: Option<BufReader<File>>,
    initial: bool,
    callbacks: Vec<&'a dyn Fn(String)>,
}

impl<'a> Default for Watcher<'a> {
    fn default() -> Self {
        Watcher {
            filename: String::new(),
            inode: 0,
            pos: 0,
            reader: None,
            initial: true,
            callbacks: vec![],
        }
    }
}

impl<'a> Watcher<'a> {
    pub fn new(filename: String) -> Watcher<'a> {
        let mut lw = Watcher::default();
        lw.filename = filename;
        lw
    }

    pub fn register<F: Fn(String)>(&mut self, callback: &'a F) {
        self.callbacks.push(callback);
    }

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
        for callback in &self.callbacks {
            callback(line.replace("\n", ""));
        }
    }

    pub fn process_all_lines(&mut self) {
        let mut line = String::new();
        loop {
            let resp = self.read_line(&mut line);
            match resp {
                Ok(0) => break,
                Ok(len) => {
                    self.pos += len as u64;
                    self.seek();
                    self.execute_callbacks(&line);
                }
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    break;
                }
            }
        }
    }

    pub fn watch(&mut self) {
        loop {
            sleep(PERIOD);
            if self.ready().is_ok() {
                self.process_all_lines();
            }
        }
    }
}
