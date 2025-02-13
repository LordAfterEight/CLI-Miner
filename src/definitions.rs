use std::{thread,time};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
#[cfg(not(target_os = "android"))]
use rodio::Decoder;
use crate::{Bytestrings, Player};

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

fn save_data(data: Player) {
    let file = match File::open("../data/saves.json") {
        Ok(file) => file,
        Err(error) => panic!("[Err] Couldn't open save file: {error:?}")
    };
    let writer = BufWriter::new(file);
}

fn read_data(player: Player) {
    let file = match File::open("../data/saves.json") {
        Ok(file) => file,
        Err(error) => panic!("[Err] Couldn't open save file: {error:?}")
    };
    let reader = BufReader::new(file);
}
