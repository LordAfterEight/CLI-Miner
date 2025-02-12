pub struct Player {
    pub nickname: String,
    pub money: f32,
    pub bits: u32,
    pub bytes: u32,
    pub miners: u32,
    pub miner_price: f32,
    pub converters: u32,
    pub converter_price: f32
}

pub struct GameState {
    pub state: String,
    pub rich_presence_state: String,
    pub progress_level: u8
}

pub struct GameSettings {
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub frame_delay: u64
}

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
