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
}

pub struct GameSettings {
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub frame_delay: u8
}
