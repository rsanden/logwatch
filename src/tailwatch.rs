use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process::{self, Command, Stdio};
use std::thread::sleep;

use crate::{Watcher, PERIOD};

pub struct LogWatcher<'a> {
    cmd: process::Child,
    reader: BufReader<process::ChildStdout>,
    callbacks: Vec<&'a dyn Fn(String)>,
}

impl<'a> LogWatcher<'a> {
    fn read_line(&mut self, mut line: &mut String) -> Result<usize, io::Error> {
        line.clear();
        self.reader.read_line(&mut line)
    }

    fn execute_callbacks(&mut self, line: &str) {
        for callback in &self.callbacks {
            callback(line.replace("\n", ""));
        }
    }

    pub fn kill(&mut self) -> Result<(), io::Error> {
        self.cmd.kill()
    }
}

impl<'a> Watcher<'a> for LogWatcher<'a> {
    fn new(filename: String) -> Self {
        let mut cmd = Command::new("tail")
            .args(&["--silent", "-n", "0", "-F", &filename])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let reader = BufReader::new(cmd.stdout.take().unwrap());
        LogWatcher {
            cmd,
            reader,
            callbacks: vec![],
        }
    }

    fn register<F: Fn(String)>(&mut self, callback: &'a F) {
        self.callbacks.push(callback);
    }

    fn watch(&mut self) {
        let mut line = String::new();
        loop {
            match self.read_line(&mut line) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    sleep(PERIOD);
                }
            };
            self.execute_callbacks(&line);
        }
    }
}
