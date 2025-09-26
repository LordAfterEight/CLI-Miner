#[derive(Debug)]
pub struct Game {
    /// Name of the game save
    save_name: String,
    /// Directory where the save is stored
    save_dir: String,
    /// Game data
    data: Data,
    /// Game settings
    settings: Settings,
    devices: Vec<Device>,
    current_screen: Screens,
}

impl Game {
    pub fn init(name: &str) -> Self {
        Self {
            save_name: name.to_string(),
            save_dir: format!("{}{}", "./data/", name),
            data: Data::init(),
            settings: Settings::init(),
            devices: Vec::new(),
            current_screen: Screens::MainMenu,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub bits: u64,
    pub bytes: u64,
    pub bytestrings: u64,
    pub money: f64,
}

impl Data {
    pub fn init() -> Self {
        Self {
            bits: 0,
            bytes: 0,
            bytestrings: 0,
            money: 60.0,
        }
    }
}

#[derive(Debug)]
/// Contains game settings like volume, difficulty, etc.
pub struct Settings {
    /// Musuc volume
    mus_vol: f32,
    /// Sound effects volume
    sfx_vol: f32,
    /// Fine-tunable difficulty level (0-255)
    difficulty: u8,
    /// Whether Discord Rich Presence is enabled or not
    discord_rich_presence: bool,
}

impl Settings {
    pub fn init() -> Self {
        Self {
            mus_vol: 0.5,
            sfx_vol: 0.5,
            difficulty: 122,
            discord_rich_presence: true,
        }
    }
}

#[derive(Debug)]
/// Basic Device base class used for all functional equipmentin the game
pub struct Device {
    /// This Device's name
    pub name: String,
    /// This Device's type (e.g. Miner)
    pub device_type: DeviceType,
    /// This Device's efficiency
    pub efficiency: u8,
    /// The devices unique ID
    pub id: u8
}

#[derive(Debug)]
/// The type of a device (e.g. Miner)
pub enum DeviceType {
    /// Mines Bits
    Miner,
    /// Converts 8 Bits to 1 Byte
    Converter,
    /// Assembles Bytes into ByteStrings
    Assembler,
    /// Analyzes ByteStrings, showing their content as UTF-8 characters
    Analyzer,
    /// Can extract certain things from ByteStrings
    Extractor,
}

#[derive(Debug)]
pub enum Screens {
    MainMenu,
    SettingsMenu,
    InGame,
    PauseMenu,
    DeviceManagement,
    GameOver,
}
