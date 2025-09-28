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

    pub fn save_game(&self, name: &str) {
        let file = std::fs::File::options().create(true).write(true).open(format!("{}{}", &self.save_dir, name));
    }

    pub fn load_game(&mut self, name: &str) {
        let file = std::fs::File::options().read(true).open(format!("{}{}", &self.save_dir, name));
    }
}

#[derive(Debug)]
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
    pub id: u8,
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
    SaveMenu,
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
        match self.is_hovered() {
            true => {
                let ret_value = is_mouse_button_released(MouseButton::Left);
                if macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p1.mp3");
                }
                if ret_value == true {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p2.mp3");
                }
                ret_value
            }
            false => false,
        }
    }

    pub fn draw(&self, font: Option<&Font>) {
        draw_button(
            &self.label,
            self.x,
            self.y,
            self.width,
            self.height,
            Alignment::Center,
            font,
        );
    }
}

pub fn draw_button(
    text: &str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    alignment: Alignment,
    font: Option<&Font>,
) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
    let text_size = 25.0;
    let text_dimensions = measure_text(text, None, text_size as u16, 1.0);
    let (text_x, text_y) = match alignment {
        Alignment::Left => (
            x + 10.0,
            y + (height + text_dimensions.height) / 2.0 + text_size / 3.3,
        ),
        Alignment::Center => (
            x + (width - text_dimensions.width) / 2.0,
            y + height / 2.0 + text_size / 3.3,
        ),
        Alignment::Right => (
            x + width - text_dimensions.width - 10.0,
            y + height / 2.0 + text_size / 3.3,
        ),
    };

    match is_hovered {
        false => {
            draw_rectangle(x, y, width, height, Color::new(0.05, 0.05, 0.05, 1.0));
            draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.6, 0.2, 0.2, 1.0));
        }
        true => {
            draw_rectangle(x, y, width, height, Color::new(0.1, 0.1, 0.1, 1.0));
            match is_mouse_button_down(MouseButton::Left) {
                false => {
                    draw_rectangle_lines(x, y, width, height, 4.0, Color::new(0.6, 0.6, 0.3, 1.0))
                }
                true => {
                    draw_rectangle_lines(x, y, width, height, 6.0, Color::new(0.3, 0.6, 0.3, 1.0))
                }
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

pub struct Audio {
    pub stream: rotilities::OutputStream,
    pub stream_handle: rotilities::OutputStreamHandle,
    pub music_sinks: Vec<rotilities::Sink>,
    pub sfx_sinks: Vec<rotilities::Sink>,
}

impl Audio {
    pub fn init() -> Self {
        let (stream, stream_handle) = rotilities::init();
        Self {
            stream: stream,
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
                    draw_texture(&self.sprite_click, x - self.sprite_click.width() / 2.0, y - self.sprite_click.height() / 2.0, WHITE);
                } else {
                    draw_texture(&self.sprite_hover, x - self.sprite_hover.width() / 2.0, y - self.sprite_hover.height() / 2.0, WHITE);
                }
            }
            false => {
                draw_texture(&self.sprite, x - self.sprite.width() / 2.0, y - self.sprite.height() / 2.0, WHITE);
            }
        }
        self.x = x;
        self.y = y;
    }
}

pub struct TextInputLabel {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub is_active: bool,
}

impl TextInputLabel {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: String::new(),
            is_active: false,
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    pub fn update(&mut self, sink_sfx: &rotilities::Sink) -> bool {
        match self.is_hovered() {
            true => {
                let ret_value = is_mouse_button_released(MouseButton::Left);
                if macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p1.mp3");
                }
                if ret_value == true {
                    rotilities::play_audio(sink_sfx, "./assets/sound/interact_p2.mp3");
                    self.is_active = !self.is_active;
                }
                ret_value
            }
            false => false,
        }
    }

    pub fn draw(&self, font: Option<&Font>) {
        let text_size = 15.0;
        let text_dimensions = measure_text(&self.text, None, text_size as u16, 1.0);
        let text_x = self.x + 10.0;
        let text_y = self.y + (self.height + text_dimensions.height) / 2.0;

        match self.is_active {
            true => {
                draw_rectangle(self.x, self.y, self.width, self.height, Color::new(0.1, 0.1, 0.1, 1.0));
                draw_rectangle_lines(self.x, self.y, self.width, self.height, 4.0, Color::new(0.3, 0.6, 0.3, 1.0));
            }
            false => {
                draw_rectangle(self.x, self.y, self.width, self.height, Color::new(0.05, 0.05, 0.05, 1.0));
                draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, Color::new(0.6, 0.2, 0.2, 1.0));
            }
        }
        draw_text_ex(
            &self.text,
            text_x,
            text_y,
            TextParams {
                font: font,
                font_size: text_size as u16,
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
        );
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub age: u16,
    pub location: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            age: 18,
            location: "".to_string()
        }
    }
}
