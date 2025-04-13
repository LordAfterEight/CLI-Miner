use serde_derive::{Serialize, Deserialize};
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub nickname: String,
    pub money: f64,
    pub bits: u64,
    pub bytes: u64,
    pub miners: u64,
    pub miner_price: f64,
    pub converters: u64,
    pub converter_price: f64
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub state: String,
    pub rich_presence_state: String,
    pub progress_level: u8
}

#[derive(Serialize, Deserialize)]
pub struct GameSettings {
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub frame_delay: u64
}

#[derive(Serialize, Deserialize)]
pub struct Bytestrings {
    pub bytestring_1: u8,
    pub bytestring_2: u8,
    pub bytestring_3: u8,
    pub bytestring_4: u8,
    pub bytestring_5: u8,
    pub bytestring_6: u8,
    pub bytestring_7: u8,
    pub bytestring_8: u8
}

pub struct Keybinds {
    pub back: KeyCode,
    pub enter: KeyCode,
    pub nav_up : KeyCode,
    pub nav_down : KeyCode,
    pub use_miner: KeyCode,
    pub use_converter: KeyCode,
    pub sell_bits: KeyCode,
    pub sell_bytes: KeyCode,
    pub buy_miner: KeyCode,
    pub buy_converter: KeyCode,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: u32,
    pub integrity: u8,
    pub efficiency: u8
}
