use std::{thread,time};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};
#[cfg(not(target_os = "android"))]
use rodio::Decoder;
use crate::{Bytestrings, Player, rand, rand::Rng};
use serde_json;
use serde_json::json;
use colored::Colorize;

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
    let filepath = "../data/save.json";
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(filepath)
        .expect("[X] Could not open file");

    file.seek(SeekFrom::Start(0)).unwrap();

    let datastruct = json!({
        "nickname" : &data.nickname.to_owned(),
        "money" : &data.money.to_owned(),
        "bits" : &data.bits.to_owned(),
        "bytes" : &data.bytes.to_owned(),
        "miners" : &data.miners.to_owned(),
        "miner_price" : &data.miner_price.to_owned(),
        "converters" : &data.converters.to_owned(),
        "converter_price" : &data.converter_price.to_owned()
    });

    serde_json::to_writer(&file, &datastruct).unwrap();
    drop(file);
}


pub fn read_data(player: &mut Player) -> &mut Player {
    let filepath = "../data/save.json";
    println!("{}", "  ┣━[i] Attempting to read data...".cyan());
    sleep(250);

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("{} {}",
            "    [X] Cannot read file: ".bold().red(),
            error
        )
    };

    let data: serde_json::Value = serde_json::from_reader(&file).expect(&"    [X] Save file must exist".bold().red());

    println!("{}", "  ┣━[i] Applying values...".cyan());
    sleep(250);

    player.nickname = data.get("nickname").expect("Value must exist").to_string();

    player.money = data.get("money").expect("Value must exist")
        .as_f64().expect("Could not convert value");

    player.miner_price = data.get("miner_price").expect("Value must exist")
        .as_f64().expect("Could not convert value");

    player.converter_price = data.get("converter_price").expect("Value must exist")
        .as_f64().expect("Could not convert value");

    player.bits = data.get("bits").expect("Value must exist")
        .as_u64().expect("Could not convert value");

    player.bytes = data.get("bytes").expect("Value must exist")
        .as_u64().expect("Could not convert value");

    player.miners = data.get("miners").expect("Value must exist")
        .as_u64().expect("Could not convert value");

    player.converters = data.get("converters").expect("Value must exist")
        .as_u64().expect("Could not convert value");

    drop(file);
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
