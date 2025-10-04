use macroquad::prelude::*;

pub async fn render_settings_screen(game: &mut crate::structs::Game) {
    let back_button =
        crate::ui::Button::new("Back", 5.0, 2.5, 100.0, 30.0, crate::ui::ButtonType::Push);
    let main_menu_button =
        crate::ui::Button::new("Main Menu", 5.0, screen_height() - 35.0, 150.0, 30.0, crate::ui::ButtonType::Push);

    let mut window = crate::ui::PopupWindow::new(
        "This is the settings menu.\nHere you can adjust your preferences like\naudio volume, difficulty, Discord Rich Presence, etc.",
        screen_width() / 2.0 - 300.0,
        screen_height() / 2.0 - 150.0,
        600.0,
        300.0,
        Vec::new(),
    );

    loop {
        draw_text_ex(
            "Settings",
            screen_width() / 2.0 - measure_text("Settings", None, 30, 1.0).width / 2.0,
            30.0,
            TextParams {
                font_size: 30,
                color: WHITE,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_text_ex(
            format!("{}", chrono::Local::now().format("%H:%M:%S%.3f")).as_str(),
            screen_width() - 150.0,
            30.0,
            TextParams {
                font_size: 15,
                color: WHITE,
                font: Some(&game.fonts[0]),
                ..Default::default()
            },
        );

        draw_line(0.0, 40.0, screen_width(), 40.0, 2.0, RED);

        if back_button.is_hovered() {
            game.cursor.hovers_clickable = true;
        } else {
            game.cursor.hovers_clickable = false;
        }

        if back_button.is_clicked(&game.audio.sfx_sinks[0]) {
            game.current_screen = game.previous_screen.take().unwrap();
            game.previous_screen = Some(crate::structs::Screens::SaveMenu);
            break;
        }

        back_button.draw(Some(&game.fonts[1]));
        if game.previous_screen == Some(crate::structs::Screens::InGame) {
            main_menu_button.draw(Some(&game.fonts[1]));
            if main_menu_button.is_clicked(&game.audio.sfx_sinks[0]) {
                game.save_game();
                game.data = crate::structs::Data::init();
                game.previous_screen = Some(crate::structs::Screens::InGame);
                game.current_screen = crate::structs::Screens::MainMenu;
                break;
            }
        }
        window.draw(Some(&game.fonts[0]));
        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}
