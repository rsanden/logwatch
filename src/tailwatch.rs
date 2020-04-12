use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process::{self, Command, Stdio};
use std::thread::sleep;

use crate::{Watcher, PERIOD};

pub struct TailWatcher<'a> {
    cmd: process::Child,
    reader: BufReader<process::ChildStdout>,
    callbacks: Vec<Box<dyn FnMut(String) + 'a>>,
}

impl<'a> TailWatcher<'a> {
    fn read_line(&mut self, mut line: &mut String) -> Result<usize, io::Error> {
        line.clear();
        self.reader.read_line(&mut line)
    }

    fn execute_callbacks(&mut self, line: &str) {
        for callback in &mut self.callbacks {
            callback(line.replace("\n", ""));
        }
    }

    pub fn kill(&mut self) -> Result<(), io::Error> {
        self.cmd.kill()
    }
}

impl<'a> Watcher<'a> for TailWatcher<'a> {
    fn new(filename: String) -> Self {
        let mut cmd = Command::new("tail")
            .args(&["--silent", "-n", "0", "-F", &filename])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let reader = BufReader::new(cmd.stdout.take().unwrap());
        TailWatcher {
            cmd,
            reader,
            callbacks: vec![],
        }
    }

    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>) {
        self.callbacks.push(callback);
    }

    fn watch(&mut self) {
        let mut line = String::new();
        loop {
            if let Err(e) = self.read_line(&mut line) {
                eprintln!("ERROR: {}", e);
                sleep(PERIOD);
            }
            self.execute_callbacks(&line);
        }
    }
}
