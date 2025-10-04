use macroquad::prelude::*;

pub async fn render_game_screen(game: &mut crate::structs::Game) {
    if game.previous_screen.take().unwrap() == crate::structs::Screens::MainMenu {
        let mut alpha = 0.0;
        let mut frames = 0;

        let mut new_volume = game.settings.mus_vol;
        while game.audio.music_sinks[0].volume() > 0.001 {
            rotilities::set_audio_volume(&game.audio.music_sinks[0], new_volume);
            new_volume *= 0.95;
            macroquad::window::next_frame().await;
        }
        rotilities::stop_audio(&game.audio.music_sinks[0]);
        rotilities::set_audio_volume(&game.audio.music_sinks[0], game.settings.mus_vol);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/music/Signal Lost.mp3");

        while alpha < 255.0 {
            draw_text_ex(
                "Prime System",
                screen_width() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).width / 2.0,
                screen_height() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).height / 2.0,
                TextParams {
                    font_size: 50,
                    color: Color::from_rgba(255, 255, 255, alpha as u8),
                    font: Some(&game.fonts[1]),
                    ..Default::default()
                },
            );
            alpha += 1.0;
            macroquad::window::next_frame().await;
        }

        while frames < 215 {
            draw_text_ex(
                "Prime System",
                screen_width() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).width / 2.0,
                screen_height() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).height / 2.0,
                TextParams {
                    font_size: 50,
                    color: Color::from_rgba(255, 255, 255, 255),
                    font: Some(&game.fonts[1]),
                    ..Default::default()
                },
            );
            frames += 1;
            macroquad::window::next_frame().await;
        }

        while alpha >= 0.01 {
            draw_text_ex(
                "Prime System",
                screen_width() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).width / 2.0,
                screen_height() / 2.0
                    - measure_text("Prime System", Some(&game.fonts[1]), 50, 1.0).height / 2.0,
                TextParams {
                    font_size: 50,
                    color: Color::from_rgba(255, 255, 255, alpha as u8),
                    font: Some(&game.fonts[1]),
                    ..Default::default()
                },
            );
            alpha *= 0.98;
            macroquad::window::next_frame().await;
        }
    }

    let mut window = crate::ui::PopupWindow::new(
        "In-Game Screen (WIP)\nHere you will spend most of your time.\n\nPress [ESC] to exit to Main Menu",
        screen_width() / 2.0 - 300.0,
        screen_height() / 2.0 - 150.0,
        600.0,
        300.0,
        Vec::new(),
    );

    loop {
        if is_key_pressed(KeyCode::Escape) {
            game.current_screen = crate::structs::Screens::SettingsMenu;
            game.previous_screen = Some(crate::structs::Screens::InGame);
            break;
        }

        game.cursor.update();
        window.draw(Some(&game.fonts[0]));
        macroquad::window::next_frame().await;
    }
}
