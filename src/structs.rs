pub struct Game {
    /// Name of the game save
    save_name: String,
    /// Directory where the save is stored
    save_dir: String,
    /// Game settings
    settings: Settings,
}

impl Game {
    pub fn init(name: &str) -> Self {
        Self {
            save_name: name.to_string(),
            save_dir: "./data/",
            settings: Settings.init(),
        }
    }
}

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

/// Basic Device base class used for all functional equipmentin the game
pub struct Device {
    /// This Device's name
    pub name: String,
    /// This Device's type (e.g. Miner)
    pub type: DeviceType,
    /// This Device's efficiency
    pub efficiency: u8,
    /// The devices unique ID
    pub id: u8
}

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

