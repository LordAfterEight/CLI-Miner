pub struct Player {
    pub nickname: String,
    pub money: f32,
    pub bits: u32,
    pub bytes: u32,
    pub miners: u32,
    pub miner_price: f32,
    pub converters: u32
}

pub struct Gamestate {
    pub state: String,
}
