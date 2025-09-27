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
    pub previous_screen: Option<Screens>,
    pub fonts: Vec<Font>,
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
            previous_screen: None,
            fonts: vec![
                load_ttf_font_from_bytes(include_bytes!("../assets/fonts/ProFont/ProFontIIxNerdFont-Regular.ttf")).unwrap(),
                load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Terminus/TerminessNerdFont-Regular.ttf")).unwrap(),
            ],
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

#[derive(Debug, PartialEq)]
pub enum Screens {
    MainMenu,
    SettingsMenu,
    InGame,
    PauseMenu,
    DeviceManagement,
    GameOver,
}

pub struct Button {
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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

    pub fn is_clicked(&self, sink_sfx: &rotilities::Sink) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        match mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height {
            true => {
                let ret_value = is_mouse_button_released(MouseButton::Left);
                if macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p1.mp3");
                }
                if ret_value == true {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p2.mp3");
                }
                ret_value
            },
            false => false,
        }
    }

    pub fn draw(&self, font: Option<&Font>) {
        draw_button(&self.label, self.x, self.y, self.width, self.height, Alignment::Center, font);
    }
}

pub fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, alignment: Alignment, font: Option<&Font>) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
    let text_size = 25.0;
    let text_dimensions = measure_text(text, None, text_size as u16, 1.0);
    let (text_x, text_y) = match alignment {
        Alignment::Left => (x + 10.0, y + (height + text_dimensions.height) / 2.0 + text_size / 3.3),
        Alignment::Center => (x + (width - text_dimensions.width) / 2.0, y + height / 2.0 + text_size / 3.3),
        Alignment::Right => (x + width - text_dimensions.width - 10.0, y + height / 2.0 + text_size / 3.3),
    };

    match is_hovered {
        false => {
            draw_rectangle(x, y, width, height, Color::new(0.05,0.05,0.05,1.0));
            draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.6,0.2,0.2,1.0));
        },
        true => {
            draw_rectangle(x, y, width, height, Color::new(0.1,0.1,0.1,1.0));
            match is_mouse_button_down(MouseButton::Left) {
                false => draw_rectangle_lines(x, y, width, height, 4.0, Color::new(0.6,0.6,0.3,1.0)),
                true => draw_rectangle_lines(x, y, width, height, 6.0, Color::new(0.3,0.6,0.3,1.0)),
            }
        }
    }
    draw_text_ex(
        text,
        text_x,
        text_y,
        TextParams {
            font: if font.is_some() { font } else { None },
            font_size: text_size as u16,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
    );
}

pub enum Alignment {
    Left,
    Center,
    Right,
}
