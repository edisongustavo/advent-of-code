use itertools::Itertools;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn contents(filename: &str) -> String {
    fs::read_to_string(filename).expect(format!("File does not exist: {filename}").as_str())
}
pub fn lines(filename: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(filename).expect(format!("File does not exist: {filename}").as_str());
    let lines = contents.split("\n");
    let vec = lines.map(|s| String::from(s)).collect_vec();
    return vec;
}

pub fn open_for_append(filename: &str) -> File {
    let path = Path::new(filename);
    if !path.exists() {
        File::create(path).unwrap().write_all(b"hello!\n").unwrap();
    }
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap()
}

pub fn append(file: &mut File, s: String) {
    writeln!(file, "{}", s).unwrap();
}
