use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use logwatch::PERIOD;

const PROG_PATH: &'static str = "target/debug/logwatch";
const MIN_DELAY: Duration = Duration::from_millis(50);

fn sleep_with_minimum(delay: Duration) {
    let mut delay = delay;
    if delay < MIN_DELAY {
        delay = MIN_DELAY;
    }
    sleep(delay);
}

fn sleep_long() {
    sleep_with_minimum(PERIOD.mul_f64(1.5));
}

fn sleep_short() {
    sleep_with_minimum(PERIOD.mul_f64(0.1));
}

fn create_file(fpath: &Path) -> File {
    if fpath.exists() {
        fs::remove_file(fpath).unwrap();
    }
    let mut fp = File::create(fpath).unwrap();
    fp.write_all(b"").unwrap();
    fp.flush().unwrap();
    fp
}

fn write_line(fp: &mut File, msg: &[u8]) {
    fp.write_all(msg).unwrap();
    fp.flush().unwrap();
}

fn read_line(reader: &mut impl BufRead) -> String {
    sleep_short();
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    s
}

#[test]
fn test_append() {
    let log_path: &Path = Path::new("tests/testlog-A");

    let mut fp = create_file(log_path);
    let mut cmd = Command::new(PROG_PATH)
        .arg(log_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        write_line(&mut fp, b"hello 1\n");
        write_line(&mut fp, b"hello 2\n");
        write_line(&mut fp, b"hello 3\n");

        sleep_long();
        let mut reader = BufReader::new(cmd.stdout.unwrap());

        write_line(&mut fp, b"hello 4\n");
        write_line(&mut fp, b"hello 5\n");
        assert_eq!(read_line(&mut reader), "hello 4\n".to_string());
        assert_eq!(read_line(&mut reader), "hello 5\n".to_string());
        write_line(&mut fp, b"hello 6\n");
        assert_eq!(read_line(&mut reader), "hello 6\n".to_string());

        cmd.stdout = Some(reader.into_inner());
    }
    cmd.kill().unwrap();
    sleep_short();
    fs::remove_file(log_path).unwrap();
}

#[test]
fn test_rotate() {
    let log1_path: &Path = Path::new("tests/testlog-B");
    let log2_path: &Path = Path::new("tests/testlog-B.1");

    let mut fp1 = create_file(log1_path);
    let mut cmd = Command::new(PROG_PATH)
        .arg(log1_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        write_line(&mut fp1, b"hello 1\n");
        write_line(&mut fp1, b"hello 2\n");
        write_line(&mut fp1, b"hello 3\n");

        sleep_long();
        let mut reader = BufReader::new(cmd.stdout.unwrap());

        write_line(&mut fp1, b"hello 4\n");
        assert_eq!(read_line(&mut reader), "hello 4\n".to_string());

        fs::rename(log1_path, log2_path).unwrap();

        let mut fp2 = create_file(log1_path);
        write_line(&mut fp2, b"hello 5\n");
        assert_eq!(read_line(&mut reader), "hello 5\n".to_string());

        cmd.stdout = Some(reader.into_inner());
    }
    cmd.kill().unwrap();
    sleep_short();
    fs::remove_file(log1_path).unwrap();
    fs::remove_file(log2_path).unwrap();
}
