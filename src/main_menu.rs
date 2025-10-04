use macroquad::prelude::*;

pub fn render_main_menu(game: &mut crate::structs::Game) {
    game.previous_screen = None;
    let (screen_w, screen_h) = (
        macroquad::window::screen_width(),
        macroquad::window::screen_height(),
    );

    let title = "NullByte »«";
    let title_font_size = 60.0;
    let subtitle = format!("V{}", env!("CARGO_PKG_VERSION"));
    let time = &format!("{}", chrono::Local::now().format("%H:%M:%S"));

    draw_text_ex(
        time,
        (screen_w - measure_text(time, Some(&game.fonts[0]), 25, 1.0).width) / 2.0,
        50.0,
        TextParams {
            font_size: 25,
            color: WHITE,
            font: Some(&game.fonts[0]),
            ..Default::default()
        },
    );

    draw_text_ex(
        title,
        (screen_w - measure_text(title, Some(&game.fonts[1]), title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 - 10.0,
        TextParams {
            font_size: title_font_size as u16,
            color: WHITE,
            font: Some(&game.fonts[1]),
            ..Default::default()
        },
    );

    draw_text_ex(
        &subtitle,
        (screen_w - measure_text(&subtitle, Some(&game.fonts[0]), 30, 1.0).width) / 2.0,
        screen_h / 4.0 + 40.0,
        TextParams {
            font_size: 30,
            color: GRAY,
            font: Some(&game.fonts[0]),
            ..Default::default()
        },
    );

    draw_line(
        (screen_w - measure_text(title, Some(&game.fonts[1]), title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 7.0,
        measure_text(title, Some(&game.fonts[1]), title_font_size as u16, 1.0).width
            + (screen_w - measure_text(title, Some(&game.fonts[1]), title_font_size as u16, 1.0).width) / 2.0,
        screen_h / 4.0 + 7.0,
        2.0,
        RED,
    );

    draw_text(
        "© Elias Stettmayer, 2025",
        screen_w - 280.0,
        screen_h - 20.0,
        25.0,
        GRAY,
    );

    let play_button = crate::ui::Button::new(
        "Play",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 - 25.0,
        200.0,
        35.0,
        crate::ui::ButtonType::Push,
    );

    let settings_button = crate::ui::Button::new(
        "Settings",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 + 25.0,
        200.0,
        35.0,
        crate::ui::ButtonType::Push,
    );

    let exit_button = crate::ui::Button::new(
        "Exit",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 + 75.0,
        200.0,
        35.0,
        crate::ui::ButtonType::Push,
    );

    let info_text: String;
    let info_text_color: Color;

    if play_button.is_hovered() {
        info_text = "Start your Adventure".to_string();
        info_text_color = GREEN;
    } else if settings_button.is_hovered() {
        info_text = "Adjust your Preferences".to_string();
        info_text_color = YELLOW;
    } else if exit_button.is_hovered() {
        info_text = "Quit the Game".to_string();
        info_text_color = RED;
    } else {
        info_text = "".to_string();
        info_text_color = BLACK;
    }

    if play_button.is_hovered() || settings_button.is_hovered() || exit_button.is_hovered() {
        game.cursor.hovers_clickable = true;
        draw_text_ex(
            &format!("{}", info_text),
            screen_w / 2.0 - measure_text(&info_text, None, 15, 1.5).width / 2.0,
            screen_h / 2.0 - 50.0,
            TextParams {
                font_size: 15,
                color: info_text_color,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );
    } else {
        game.cursor.hovers_clickable = false;
    }

    play_button.draw(Some(&game.fonts[1]));
    settings_button.draw(Some(&game.fonts[1]));
    exit_button.draw(Some(&game.fonts[1]));

    if exit_button.is_clicked(&game.audio.sfx_sinks[0]) {
        std::thread::sleep(std::time::Duration::from_millis(400));
        std::process::exit(0);
    }

    if settings_button.is_clicked(&game.audio.sfx_sinks[0]) {
        game.current_screen = crate::structs::Screens::SettingsMenu;
        game.previous_screen = Some(crate::structs::Screens::MainMenu);
    }
    if play_button.is_clicked(&game.audio.sfx_sinks[0]) {
        game.current_screen = crate::structs::Screens::SaveMenu;
        game.previous_screen = Some(crate::structs::Screens::MainMenu);
    }
}
