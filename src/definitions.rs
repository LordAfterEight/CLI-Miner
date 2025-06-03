use std::{thread,time};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use crate::{Bytestrings, Player, GameSettings, Device, rand, rand::Rng};
use serde_json;
use serde_json::{json, Value};
use colored::Colorize;

pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

pub fn binary_to_string(byte: u8) -> String {
    let string = char::from(byte).to_string();
    return string
}


pub fn save_player_data(data: &mut Player) {
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


pub fn read_player_data(player: &mut Player) -> &mut Player {
    println!("{}", "  ┣━[i] Attempting to read data...".cyan());
    sleep(250);

    let playerdata = match File::open("../data/save.json") {
        Ok(file) => file,
        Err(error) => panic!("{} {}\n{}",
            "    [X] Error while opening file".bold().red(),
            error,
            "Check if 'save.json' is at CLI-Miner/data/".yellow()
        )
    };

    let data: serde_json::Value = serde_json::from_reader(&playerdata).expect(&"    [X] Save file must exist".bold().red());

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
        .as_u64().expect("Could not convert value") as usize;

    player.bytes = data.get("bytes").expect("Value must exist")
        .as_u64().expect("Could not convert value") as usize;

    player.miners = data.get("miners").expect("Value must exist")
        .as_u64().expect("Could not convert value") as usize;

    player.converters = data.get("converters").expect("Value must exist")
        .as_u64().expect("Could not convert value") as usize;

    drop(playerdata);
    return player
}


pub fn save_settings_data(settings: &mut GameSettings) {
    let filepath = "../data/settings.json";
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(filepath)
        .expect("[X] Could not open file");

    file.seek(SeekFrom::Start(0)).unwrap();

    let datastruct = json!({
        "sfx_volume" : &settings.sfx_volume.to_owned(),
        "music_volume" : &settings.music_volume.to_owned(),
        "frame_delay" : &settings.frame_delay.to_owned()
    });

    serde_json::to_writer(&file, &datastruct).unwrap();
    drop(file);
}

pub fn read_settings_data(
    settings: &mut GameSettings
) -> &mut GameSettings {
    let filepath = "../data/settings.json";
    println!("{}", "  ┣━[i] Attempting to read settings file...".cyan());
    sleep(250);

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("{} {}\n{}",
            "    [X] Error while opening file".bold().red(),
            error,
            "Check if 'settings.json' is at CLI-Miner/data/".yellow()
        )
    };

    let data: serde_json::Value = serde_json::from_reader(&file).expect(&"    [X] Settings file error".bold().red());

    println!("{}", "  ┗━[i] Applying values...".cyan());
    sleep(250);

    settings.sfx_volume = data.get("sfx_volume").expect("Value must exist")
        .as_f64().expect("Could not convert value") as f32;

    settings.music_volume = data.get("music_volume").expect("Value must exist")
        .as_f64().expect("Could not convert value") as f32;

    settings.frame_delay = data.get("frame_delay").expect("Value must exist")
        .as_u64().expect("Could not convert value");


    drop(file);
    return settings
}

pub fn save_gamedata(miner_list: &mut Vec<Device>, player: &mut Player) {
    let filepath = "../data/gamedata.json";
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(filepath)
        .expect("[X] Could not open file");

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut datastruct: Vec<Value> = Vec::new();
    let data = json!({
        format!("miner{}", 1) : miner_list[0].id.to_owned(),
        format!("miner{}", 2) : miner_list[1].id.to_owned(),
        format!("miner{}", 3) : miner_list[2].id.to_owned(),
        format!("miner{}", 4) : miner_list[3].id.to_owned(),
        format!("miner{}", 5) : miner_list[4].id.to_owned(),
        format!("miner{}", 6) : miner_list[5].id.to_owned(),
        format!("miner{}", 7) : miner_list[6].id.to_owned(),
        format!("miner{}", 8) : miner_list[7].id.to_owned(),
        format!("miner{}", 9) : miner_list[8].id.to_owned(),
        format!("miner{}", 10) : miner_list[9].id.to_owned(),
        format!("miner{}", 11) : miner_list[10].id.to_owned(),
        format!("miner{}", 12) : miner_list[11].id.to_owned(),
        format!("miner{}", 13) : miner_list[12].id.to_owned(),
        format!("miner{}", 14) : miner_list[13].id.to_owned(),
        format!("miner{}", 15) : miner_list[14].id.to_owned(),
        format!("miner{}", 16) : miner_list[15].id.to_owned()
    });

    serde_json::to_writer(&file, &data).unwrap();

    drop(file);
}

pub fn read_gamedata(player: &mut Player) -> Vec<Device> {
    if player.miners > 0 {
        let mut device_ids: Vec<u32> = vec![00000;20];
        let file = match File::open("../data/gamedata.json") {
            Ok(file) => file,
            Err(error) => panic!("{} {}\n{}",
                "    [X] Error while opening file: ".bold().red(),
                error,
                "Check if 'gamedata.json' is at CLI-Miner/data/".yellow()
            )
        };

        let miner: serde_json::Value = serde_json::from_reader(&file).expect(&"    [X] Gamedata file error".bold().red());

        for i in 0..player.miners {
            device_ids[i] = miner.get(format!("miner{}", i+1)).expect("Value must exist")
                .as_u64().expect("Could not convert Value") as u32;
            //println!("Fetched ID of miner {}: {}",&i, &device_ids[i]);
        }

        drop(file);
        println!("{}", "  ┣━[i] Loading devices...".cyan());

        let mut miner_list = Vec::<Device>::new();
        for i in 0..20 {
            miner_list.push(
                Device{
                    id: device_ids[i],
                    integrity: 100,
                    efficiency: 1
                }
            )
        }

        return miner_list
    } else {
        println!("{}", "  ┣━[i] No devices to load".yellow());
        let mut miner_list = Vec::<Device>::new();
        return miner_list
    }
}
/*
pub fn save_keybinds_data(keybinds: &mut Keybinds) {
    let filepath = "../data/keybinds.json";
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(filepath)
        .expect("[X] Could not open file");

    file.seek(SeekFrom::Start(0)).unwrap();

    let datastruct = json!({
        "back" : &keybinds.back.to_owned(),
        "enter" : &keybinds.enter.to_owned(),
        "nav_up" : &keybinds.nav_up.to_owned()
    });

    serde_json::to_writer(&file, &datastruct).unwrap();
    drop(file);
}

pub fn read_keybinds_data(
    keybinds: &mut Keybinds
) -> &mut Keybinds {
    let filepath = "../data/keybinds.json";
    println!("{}", "  ┣━[i] Attempting to read keybinds file...".cyan());
    sleep(250);

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("{} {}\n{}",
            "    [X] Error while opening file: ".bold().red(),
            error,
            "Check if 'keybinds.json' is at CLI-Miner/data/".yellow()
        )
    };

    let data: serde_json::Value = serde_json::from_reader(&file).expect(&"    [X] Keybinds file must exist".bold().red());

    println!("{}", "  ┗━[i] Applying values...".cyan());
    sleep(250);

    keybinds.back = data.get("back").expect("Value must exist")
        .as_char().expect("Could not convert value");

    keybinds.enter = data.get("enter").expect("Value must exist")
        .as_char().expect("Could not convert value");

    keybinds.nav_up = data.get("nav_up").expect("Value must exist")
        .as_char().expect("Could not convert value");


    drop(file);
    return keybinds
}
*/

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
