use std::{thread,time};
use std::fs::File;
use std::io::BufReader;
#[cfg(not(target_os = "android"))]
use rodio::Decoder;
use crate::Bytestrings;

pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

#[cfg(not(target_arch = "aarch64"))]
pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(BufReader::new(File::open(format!("../sound/{filename}")).expect("failed to read file")));
    return source.expect("failed to decode file")
}

pub fn binary_to_string(byte: u8) -> String {
    let string = char::from(byte).to_string();
    return string
}
