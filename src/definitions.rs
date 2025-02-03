use std::{thread,time};
use std::fs::File;
use std::io::BufReader;
use rodio::Decoder;
pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(BufReader::new(File::open(format!("../sound/{filename}")).expect("failed to read file")));
    return source.expect("failed to decode file")
}
