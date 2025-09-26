use macroquad::prelude::*;

use crate::structs::*;

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

    let play_button = Button::new("Start Game", screen_w / 2.0 - 100.0, screen_h / 2.0 - 25.0, 200.0, 40.0);
    let settings_button = Button::new("Settings", screen_w / 2.0 - 100.0, screen_h / 2.0 + 35.0, 200.0, 40.0);
    let exit_button = Button::new("Exit", screen_w / 2.0 - 100.0, screen_h / 2.0 + 95.0, 200.0, 40.0);

    play_button.draw();
    settings_button.draw();
    exit_button.draw();

    if exit_button.is_clicked(mouse_position().0, mouse_position().1) && is_mouse_button_down(MouseButton::Left) {
        std::process::exit(0);
    }
}

pub fn render_settings_screen() {
}

pub fn render_game_screen() {
}

