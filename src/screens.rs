use macroquad::prelude::*;

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub fn render_main_menu() {
    let (screen_w, screen_h) = (macroquad::window::screen_width(), macroquad::window::screen_height());

    let title = "CLI-MINER »«";
    let title_font_size = 60.0;

    draw_text(
        title,
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0,
        title_font_size,
        WHITE,
    );

    draw_line(
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 10.0,
        measure_text(title, None, title_font_size as u16, 1.0).width + (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 10.0,
        2.0,
        RED,
    );

    draw_text("[Q] to Exit", 20.0, 20.0, 20.0, GRAY);
    draw_text("© Elias Stettmayer, 2025", screen_w - 280.0, screen_h - 20.0, 25.0, GRAY);
}

pub fn render_settings_screen() {
}

pub fn render_game_screen() {
}

