use macroquad::prelude::*;
use macroquad::rand;
use rotilities;

use crate::screens::render_game_screen;

mod screens;
mod structs;
mod input;

fn setup_window() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "CLI-Miner".to_owned(),
        window_width: (1920.0) as i32,
        window_height: (1080.0) as i32,
        high_dpi: false,
        window_resizable: false,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(setup_window())]
async fn main() {
    macroquad::input::show_mouse(false);
    let mut game = crate::structs::Game::init("LordAfterEight").await;

    game.audio.music_sinks.push(rotilities::new_sink(&game.audio.stream_handle));
    game.audio.sfx_sinks.push(rotilities::new_sink(&game.audio.stream_handle));

    loop {
        match game.current_screen {
            crate::structs::Screens::MainMenu => {
                crate::screens::render_main_menu(&mut game);
                if game.audio.music_sinks[0].empty() {
                    rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/Main Menu.mp3");
                }
            },
            crate::structs::Screens::SaveMenu => {
                crate::screens::render_save_menu(&mut game);
                if game.audio.music_sinks[0].empty() {
                    rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/Main Menu.mp3");
                }
            },
            crate::structs::Screens::SettingsMenu => {
                crate::screens::render_settings_screen(&mut game);
                if game.audio.music_sinks[0].empty() {
                    rotilities::play_audio(&game.audio.music_sinks[0], "./assets/sound/Main Menu.mp3");
                }
            },
            crate::structs::Screens::InGame => {
                let mut new_volume = game.settings.mus_vol;
                while game.audio.music_sinks[0].volume() > 0.0 {
                    rotilities::set_audio_volume(&game.audio.music_sinks[0], new_volume);
                    new_volume -= 0.002;
                    macroquad::window::next_frame().await;
                }
                rotilities::stop_audio(&game.audio.music_sinks[0]);
                rotilities::set_audio_volume(&game.audio.music_sinks[0], game.settings.mus_vol);
                std::thread::sleep(std::time::Duration::from_millis(1000));

                while game.current_screen == crate::structs::Screens::InGame {
                    render_game_screen(&mut game);
                    macroquad::window::next_frame().await;
                }
            },

            _ => {}
        }

        game.cursor.update();
        macroquad::window::next_frame().await;
    }
}
