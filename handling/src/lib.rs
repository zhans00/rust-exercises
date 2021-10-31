use std::io::Write;
use std::fs::OpenOptions;

pub fn open_or_create(s: &str, content: &str) {
	let mut f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(s)
        .expect("Unable to open file");
    f.write_all(content.as_bytes()).expect("Unable to write data");
}