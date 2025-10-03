use std::io::Write;

use macroquad::prelude::*;
pub struct Game {
    /// Directory where the save is stored
    pub save_dir: String,
    /// Game data
    pub data: Data,
    /// Game settings
    pub settings: Settings,
    pub devices: Vec<Device>,
    pub current_screen: Screens,
    pub previous_screen: Option<Screens>,
    pub fonts: Vec<Font>,
    pub audio: Audio,
    pub cursor: Cursor,
}

impl Game {
    pub async fn init() -> Self {
        Self {
            save_dir: "./data/saves/".to_string(),
            data: Data::init(),
            settings: Settings::init(),
            devices: Vec::new(),
            current_screen: Screens::MainMenu,
            previous_screen: None,
            fonts: vec![
                load_ttf_font_from_bytes(include_bytes!(
                    "../assets/fonts/ProFont/ProFontIIxNerdFont-Regular.ttf"
                ))
                .unwrap(),
                load_ttf_font_from_bytes(include_bytes!(
                    "../assets/fonts/Terminus/TerminessNerdFont-Regular.ttf"
                ))
                .unwrap(),
            ],
            audio: Audio::init(),
            cursor: Cursor::new().await,
        }
    }

    pub fn save_game(&self) {
        let path = format!("{}{}", &self.save_dir, &self.data.player.name);
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(&path)
            .expect(&format!("File at \"{}\" could not be opened", path));

        let data = serde_json::to_string_pretty(&serde_json::json!([&self.data, &self.settings])).unwrap();

        _ = file.write(data.as_bytes());
    }

    pub fn load_game(&mut self, name: &str) {
        let path = format!("{}{}", &self.save_dir, &self.data.player.name);
        let file = std::fs::File::options()
            .read(true)
            .open(&path)
            .expect(&format!("File at \"{}\" could not be opened", path));
        let reader = std::io::BufReader::new(file);

        let objects: Vec<serde_json::Value> = serde_json::from_reader(reader).unwrap();

        self.data = serde_json::from_value(objects[0].clone()).unwrap();
        self.settings = serde_json::from_value(objects[1].clone()).unwrap();
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub player: Player,
    pub bits: u64,
    pub bytes: u64,
    pub bytestrings: u64,
    pub money: f64,
}

impl Data {
    pub fn init() -> Self {
        Self {
            player: Player::new(),
            bits: 0,
            bytes: 0,
            bytestrings: 0,
            money: 60.0,
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
/// Contains game settings like volume, difficulty, etc.
pub struct Settings {
    /// Musuc volume
    pub mus_vol: f32,
    /// Sound effects volume
    pub sfx_vol: f32,
    /// Fine-tunable difficulty level (0-255)
    pub difficulty: u8,
    /// Whether Discord Rich Presence is enabled or not
    pub discord_rich_presence: bool,
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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
/// Basic Device base class used for all functional equipmentin the game
pub struct Device {
    /// This Device's name
    pub name: String,
    /// This Device's type (e.g. Miner)
    pub device_type: DeviceType,
    /// This Device's efficiency
    pub efficiency: u8,
    /// The devices unique ID
    pub id: u8,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
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

#[derive(Debug, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Screens {
    MainMenu,
    SaveMenu,
    SettingsMenu,
    InGame,
    PauseMenu,
    DeviceManagement,
    GameOver,
}

pub struct Audio {
    pub _stream: rotilities::OutputStream,
    pub stream_handle: rotilities::OutputStreamHandle,
    pub music_sinks: Vec<rotilities::Sink>,
    pub sfx_sinks: Vec<rotilities::Sink>,
}

impl Audio {
    pub fn init() -> Self {
        let (stream, stream_handle) = rotilities::init();
        Self {
            _stream: stream,
            stream_handle: stream_handle,
            music_sinks: Vec::new(),
            sfx_sinks: Vec::new(),
        }
    }
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub sprite: Texture2D,
    pub sprite_hover: Texture2D,
    pub sprite_click: Texture2D,
    pub hovers_clickable: bool,
}

impl Cursor {
    pub async fn new() -> Self {
        let (x, y) = mouse_position();
        Self {
            x,
            y,
            sprite: load_texture("./assets/sprites/Cursor.png")
                .await
                .expect("Failed to load cursor sprite"),
            sprite_hover: load_texture("./assets/sprites/Cursor_hover.png")
                .await
                .expect("Failed to load cursor hover sprite"),
            sprite_click: load_texture("./assets/sprites/Cursor_click.png")
                .await
                .expect("Failed to load cursor click sprite"),
            hovers_clickable: false,
        }
    }

    pub fn update(&mut self) {
        let (x, y) = mouse_position();
        match self.hovers_clickable {
            true => {
                if is_mouse_button_down(MouseButton::Left) {
                    draw_texture(
                        &self.sprite_click,
                        x - self.sprite_click.width() / 2.0,
                        y - self.sprite_click.height() / 2.0,
                        WHITE,
                    );
                } else {
                    draw_texture(
                        &self.sprite_hover,
                        x - self.sprite_hover.width() / 2.0,
                        y - self.sprite_hover.height() / 2.0,
                        WHITE,
                    );
                }
            }
            false => {
                draw_texture(
                    &self.sprite,
                    x - self.sprite.width() / 2.0,
                    y - self.sprite.height() / 2.0,
                    WHITE,
                );
            }
        }
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Player {
    pub name: String,
    pub age: u16,
    pub location: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Player".to_string(),
            age: 18,
            location: "".to_string(),
        }
    }
}
