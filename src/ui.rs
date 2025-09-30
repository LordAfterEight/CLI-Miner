use macroquad::prelude::*;
use crate::screens::Alignment;

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
            font: font,
            font_size: text_size as u16,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
    );
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

    /// Handles any global keyboard input. Returns a bool if Enter was pressed
    pub fn use_input(&mut self, game: &mut crate::structs::Game) -> bool {
        if is_key_pressed(KeyCode::Backspace) {
            self.text.pop();
        }
        if is_key_pressed(KeyCode::Enter) {
            game.data.player.name = self.text.clone();
            game.current_screen = crate::structs::Screens::InGame;
            game.previous_screen = Some(crate::structs::Screens::SaveMenu);
            return true
        }
        match macroquad::input::get_char_pressed() {
            Some(c) => {
                self.text.push(c);
            },
            None => {}
        }
        return false
    }
}
