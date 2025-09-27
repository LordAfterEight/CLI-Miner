use macroquad::prelude::*;
use crate::structs::*;

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub fn render_main_menu(game: &mut Game, sink_sfx: &rotilities::Sink) {
    let (screen_w, screen_h) = (macroquad::window::screen_width(), macroquad::window::screen_height());

    let title = "CLI-MINER »«";

    let title_font_size = 60.0;

    let subtitle = format!("V{}", env!("CARGO_PKG_VERSION"));

    draw_text_ex(
        title,
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0 - 25.0,
        screen_h / 4.0,
        TextParams {
            font_size: title_font_size as u16,
            color: WHITE,
            font: Some(&game.fonts[1]),
            ..Default::default()
        },
    );

    draw_text_ex(
        &subtitle,
        (screen_w - measure_text(&subtitle, None, 30, 1.0).width) / 2.0 - 20.0,
        screen_h / 4.0 + 40.0,
        TextParams {
            font_size: 30,
            color: GRAY,
            font: Some(&game.fonts[0]),
            ..Default::default()
        },
    );

    draw_line(
        (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0 - 25.0,
        screen_h / 4.0 + 10.0,
        measure_text(title, None, title_font_size as u16, 1.0).width + (screen_w - measure_text(title, None, title_font_size as u16, 1.0).width) / 2.0 + 25.0,
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
        draw_text_ex(
            "Start your adventure",
            screen_w / 2.0 - measure_text("Start your adventure", None, 15, 1.5).width / 2.0,
            screen_h / 2.0 - 50.0,
            TextParams {
                font_size: 15,
                color: GREEN,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );
    }

    if settings_button.is_hovered() {
        draw_text_ex(
            "Adjust your preferences",
            screen_w / 2.0 - measure_text("Adjust your preferences", None, 15, 1.5).width / 2.0,
            screen_h / 2.0 - 50.0,
            TextParams {
                font_size: 15,
                color: YELLOW,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );
    }

    if exit_button.is_hovered() {
        draw_text_ex(
            "Quit the game",
            screen_w / 2.0 - measure_text("Quit the game", None, 15, 1.5).width / 2.0,
            screen_h / 2.0 - 50.0,
            TextParams {
                font_size: 15,
                color: RED,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );
    }

    play_button.draw(Some(&game.fonts[1]));
    settings_button.draw(Some(&game.fonts[1]));
    exit_button.draw(Some(&game.fonts[1]));

    if exit_button.is_clicked(&sink_sfx) {
        std::thread::sleep(std::time::Duration::from_millis(400));
        std::process::exit(0);
    }

    if settings_button.is_clicked(&sink_sfx) {
        game.current_screen = Screens::SettingsMenu;
        game.previous_screen = Some(Screens::MainMenu);
    }
    if play_button.is_clicked(&sink_sfx) {
        game.current_screen = Screens::InGame;
        game.previous_screen = Some(Screens::MainMenu);
    }
}

pub fn render_settings_screen() {
}

pub fn render_game_screen(game: &mut Game) {
    let (screen_w, screen_h) = (macroquad::window::screen_width(), macroquad::window::screen_height());

    draw_text_ex(
        "In-Game Screen",
        20.0,
        40.0,
        TextParams {
            font_size: 30,
            color: WHITE,
            font: Some(&game.fonts[1]),
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press 'Esc' to return to Main Menu",
        20.0,
        80.0,
        TextParams {
            font_size: 20,
            color: GRAY,
            font: Some(&game.fonts[0]),
            ..Default::default()
        },
    );

    if is_key_pressed(KeyCode::Escape) {
        game.current_screen = Screens::MainMenu;
        game.previous_screen = Some(Screens::InGame);
    }
}
