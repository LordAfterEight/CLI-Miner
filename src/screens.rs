use crate::structs::*;
use macroquad::prelude::*;

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub fn render_main_menu(game: &mut Game) {
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
        game.current_screen = Screens::SettingsMenu;
        game.previous_screen = Some(Screens::MainMenu);
    }
    if play_button.is_clicked(&game.audio.sfx_sinks[0]) {
        game.current_screen = Screens::SaveMenu;
        game.previous_screen = Some(Screens::MainMenu);
    }
}

pub async fn render_save_menu(game: &mut Game) {
    let exit_button =
        crate::ui::Button::new("Back", 5.0, 2.5, 100.0, 30.0, crate::ui::ButtonType::Push);

    let mut col1 = WHITE;
    let mut col2 = GRAY;

    let newgame_label_x_alignment = screen_width() / 2.0 + 250.0;

    let mut name_label = crate::ui::TextInputLabel::new(
        Some("Name:".to_string()),
        newgame_label_x_alignment,
        screen_height() / 3.0 - 20.0,
        300.0,
        30.0,
    );

    let mut age_label = crate::ui::TextInputLabel::new(
        Some("Age:".to_string()),
        newgame_label_x_alignment,
        screen_height() / 3.0 + 20.0,
        300.0,
        30.0,
    );


    while game.current_screen == Screens::SaveMenu {
        let mut saves = Vec::<crate::ui::Button>::new();
        let mut del_btns = Vec::<crate::ui::Button>::new();
        let mut i = 0.0;
        for save in std::fs::read_dir("./data/saves/").unwrap() {
            let save = save.unwrap();
            let save_path = save.path();

            if save_path.is_file() {
                saves.push(crate::ui::Button::new(
                    &format!("{}", save_path.to_str().unwrap().trim_start_matches("./data/saves/")),
                    400.0,
                    300.0 + i * 35.0,
                    250.0,
                    30.0,
                    crate::ui::ButtonType::Push,
                ));
                del_btns.push(crate::ui::Button::new(
                    "Delete",
                    250.0,
                    300.0 + i * 35.0,
                    100.0,
                    30.0,
                    crate::ui::ButtonType::Push,
                ));
            }

            i += 1.0;
        }

        draw_text_ex(
            "Saves",
            screen_width() / 2.0 - measure_text("Saves", Some(&game.fonts[1]), 30, 1.0).width / 2.0,
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

        draw_text_ex(
            "Load Game",
            screen_width() / 4.0 - measure_text("Load Game", Some(&game.fonts[1]), 25, 1.0).width / 2.0,
            180.0,
            TextParams {
                font_size: 25,
                color: col1,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_text_ex(
            "New Game",
            3.0 * screen_width() / 4.0 - measure_text("New Game", Some(&game.fonts[1]), 25, 1.0).width / 2.0,
            180.0,
            TextParams {
                font_size: 25,
                color: col2,
                font: Some(&game.fonts[1]),
                ..Default::default()
            },
        );

        draw_line(0.0, 40.0, screen_width(), 40.0, 2.0, RED);

        draw_line(
            screen_width() / 2.0,
            240.0,
            screen_width() / 2.0,
            screen_height() - 200.0,
            2.0,
            RED,
        );

        if exit_button.is_hovered() {
            game.cursor.hovers_clickable = true;
        } else {
            game.cursor.hovers_clickable = false;
        }

        if game.cursor.x > screen_width() / 2.0 {
            draw_line(
                3.0 * screen_width() / 4.0 - 50.0,
                190.0,
                3.0 * screen_width() / 4.0 + 62.0,
                190.0,
                4.0,
                WHITE,
            );
            col1 = GRAY;
            col2 = WHITE;
        } else {
            draw_line(
                screen_width() / 4.0 - 50.0,
                190.0,
                screen_width() / 4.0 + 65.0,
                190.0,
                4.0,
                WHITE,
            );
            col1 = WHITE;
            col2 = GRAY;
        }

        if exit_button.is_clicked(&game.audio.sfx_sinks[0]) {
            game.current_screen = game.previous_screen.take().unwrap();
            game.previous_screen = Some(Screens::SaveMenu);
            break;
        }

        if name_label.is_hovered() || age_label.is_hovered() {
            game.cursor.hovers_clickable = true;
        }

        let (enter_pressed, name_label_text) = name_label.use_input(game);
        let age_label_text = age_label.use_input(game).1;

        if name_label.is_active {
            game.data.player.name = match name_label_text {
                Some(c) => c,
                None => "Player".to_string(),
            };
        }

        if age_label.is_active {
            game.data.player.age = match age_label_text {
                Some(c) => {
                    if c.len() > 0 {
                        c.parse().unwrap()
                    } else {
                        18
                    }
                }
                None => 18,
            };
        }

        if enter_pressed && name_label.text.len() > 0 {
            game.current_screen = Screens::InGame;
            game.cursor.hovers_clickable = false;
            game.create_game_file();
            break;
        }

        name_label.update(&game.audio.sfx_sinks[0]);
        name_label.draw(Some(&game.fonts[0]));
        age_label.update(&game.audio.sfx_sinks[0]);
        age_label.draw(Some(&game.fonts[0]));
        exit_button.draw(Some(&game.fonts[1]));
        for i in 0..saves.len() {
            saves[i].draw(Some(&game.fonts[1]));
            if saves[0].is_clicked(&game.audio.sfx_sinks[0]) {
                game.load_game(saves[0].label.trim_start_matches("./data/saves/"));
                game.current_screen = Screens::InGame;
                break;
            }
            del_btns[i].draw(Some(&game.fonts[1]));
            if del_btns[i].is_clicked(&game.audio.sfx_sinks[0]) {
                match std::fs::remove_file(format!("./data/saves/{}", saves[i].label)) {
                    Ok(_) => {},
                    Err(_) => rotilities::play_audio(&game.audio.sfx_sinks[0], "./assets/sound/sfx/fail.mp3")
                }
            }
        }
        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}

pub async fn render_settings_screen(game: &mut Game) {
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
            game.previous_screen = Some(Screens::SaveMenu);
            break;
        }

        back_button.draw(Some(&game.fonts[1]));
        if game.previous_screen == Some(Screens::InGame) {
            main_menu_button.draw(Some(&game.fonts[1]));
            if main_menu_button.is_clicked(&game.audio.sfx_sinks[0]) {
                game.save_game();
                game.data = crate::structs::Data::init();
                game.previous_screen = Some(Screens::InGame);
                game.current_screen = Screens::MainMenu;
                break;
            }
        }
        window.draw(Some(&game.fonts[0]));
        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}

pub async fn render_game_screen(game: &mut Game) {
    if game.previous_screen.take().unwrap() == Screens::MainMenu {
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
            game.current_screen = Screens::SettingsMenu;
            game.previous_screen = Some(Screens::InGame);
            break;
        }

        game.cursor.update();
        window.draw(Some(&game.fonts[0]));
        macroquad::window::next_frame().await;
    }
}
