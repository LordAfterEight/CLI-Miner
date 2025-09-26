use macroquad::prelude::*;

#[derive(Debug)]
pub struct Game {
    /// Name of the game save
    pub save_name: String,
    /// Directory where the save is stored
    pub save_dir: String,
    /// Game data
    pub data: Data,
    /// Game settings
    pub settings: Settings,
    pub devices: Vec<Device>,
    pub current_screen: Screens,
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

pub struct Button {
    label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            label: label.to_string(),
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    pub fn is_clicked(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        match mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height {
            true => is_mouse_button_released(MouseButton::Left),
            false => false,
        }
    }

    pub fn draw(&self) {
        draw_button(&self.label, self.x, self.y, self.width, self.height, Alignment::Center);
    }
}

pub fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, alignment: Alignment) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
    let text_size = 30.0;
    let text_dimensions = measure_text(text, None, text_size as u16, 1.0);
    let (text_x, text_y) = match alignment {
        Alignment::Left => (x + 10.0, y + (height + text_dimensions.height) / 2.0),
        Alignment::Center => (x + (width - text_dimensions.width) / 2.0, y + (height + text_dimensions.height) / 2.0),
        Alignment::Right => (x + width - text_dimensions.width - 10.0, y + (height + text_dimensions.height) / 2.0),
    };

    match is_hovered {
        false => {
            draw_rectangle(x, y, width, height, Color::new(0.1,0.1,0.1,1.0));
            draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.6,0.2,0.2,1.0));
        },
        true => {
            draw_rectangle(x, y, width, height, Color::new(0.15,0.15,0.15,1.0));
            match is_mouse_button_down(MouseButton::Left) {
                false => draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.6,0.6,0.3,1.0)),
                true => draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.3,0.6,0.3,1.0)),
            }
        }
    }
    draw_text(text, text_x, text_y, text_size, WHITE);
}

pub enum Alignment {
    Left,
    Center,
    Right,
}
