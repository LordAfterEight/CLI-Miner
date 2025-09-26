use macroquad::prelude::*;
use crate::structs::*;

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub fn render_main_menu() {
    let (screen_w, screen_h) = (macroquad::window::screen_width(), macroquad::window::screen_height());

    let title = "CLI-MINER »«";
    let title_font_size = 60.0;

    let subtitle = format!("V{}", env!("CARGO_PKG_VERSION"));

    draw_text(
        title,
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0,
        title_font_size,
        WHITE,
    );

    draw_text(
        &subtitle,
        (screen_w - measure_text(&subtitle, None, 30, 1.0).width) / 2.0,
        screen_h / 4.0 + 40.0,
        30.0,
        LIGHTGRAY,
    );

    draw_line(
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 10.0,
        measure_text(title, None, title_font_size as u16, 1.0).width + (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 10.0,
        2.0,
        RED,
    );

    draw_text("© Elias Stettmayer, 2025", screen_w - 280.0, screen_h - 20.0, 25.0, GRAY);

    let play_button = crate::structs::Button::new(
        "Play",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 - 25.0,
        200.0,
        35.0,
    );

    let settings_button = crate::structs::Button::new(
        "Settings",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 + 25.0,
        200.0,
        35.0,
    );

    let exit_button = crate::structs::Button::new(
        "Exit",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 + 75.0,
        200.0,
        35.0,
    );

    if play_button.is_hovered() {
        draw_text(
            "Start the game",
            screen_w / 2.0 - measure_text("Start the game", None, 25, 1.0).width / 2.0,
            screen_h / 2.0 - 60.0,
            25.0,
            SKYBLUE,
        );
    }

    if settings_button.is_hovered() {
        draw_text(
            "Adjust your settings",
            screen_w / 2.0 - measure_text("Adjust your settings", None, 25, 1.0).width / 2.0,
            screen_h / 2.0 - 60.0,
            25.0,
            YELLOW,
        );
    }

    if exit_button.is_hovered() {
        draw_text(
            "Exit the game",
            screen_w / 2.0 - measure_text("Exit the game", None, 25, 1.0).width / 2.0,
            screen_h / 2.0 - 60.0,
            25.0,
            RED,
        );
    }

    play_button.draw();
    settings_button.draw();
    exit_button.draw();

    if exit_button.is_clicked() {
        std::process::exit(0);
    }
}

pub fn render_settings_screen() {
}

pub fn render_game_screen() {
}
