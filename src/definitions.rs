use std::{thread,time};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
#[cfg(not(target_os = "android"))]
use rodio::Decoder;
use crate::{Bytestrings, Player, rand, rand::Rng};

pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

#[cfg(not(target_arch = "aarch64"))]
pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(
        BufReader::new(
            File::open(format!("../sound/{filename}"))
                .expect("failed to read file")
        )
    );
    return source.expect("failed to decode file")
}

pub fn binary_to_string(byte: u8) -> String {
    let string = char::from(byte).to_string();
    return string
}

pub fn save_data(data: &mut Player) {
    let filepath = "../data/saves.txt";
    let file = File::open(filepath).expect("[X] Could not open file");
}

pub fn read_data(player: &mut Player) -> &mut Player {
    let filepath = "../data/saves.txt";
    let mut file = File::open(filepath).expect("[X] Could not open file");
    let mut content = String::new();
    println!("[i] Attempting to read saves file...");
    sleep(250);
    player.nickname = file.read_to_string(&mut content)
        .expect("[X] Could not read file")
        .to_string();
    return player
}

pub fn generate_bytes(object: &mut Bytestrings) -> &mut Bytestrings{
    object.bytestring_1 = rand::rng().random();
    object.bytestring_2 = rand::rng().random();
    object.bytestring_3 = rand::rng().random();
    object.bytestring_4 = rand::rng().random();
    object.bytestring_5 = rand::rng().random();
    object.bytestring_6 = rand::rng().random();
    object.bytestring_7 = rand::rng().random();
    object.bytestring_8 = rand::rng().random();
    return object;
}
