use std::io::sink;

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
    let mut game = crate::structs::Game::init("LordAfterEight");
    let (_stream, stream_handle) = rotilities::init();
    let sink_music = rotilities::new_sink(&stream_handle);
    let sink_sfx = rotilities::new_sink(&stream_handle);

    rotilities::set_audio_volume(&sink_music, game.settings.mus_vol);
    rotilities::set_audio_volume(&sink_sfx, game.settings.sfx_vol);

    println!("Game: {:#?}", game);

    rotilities::play_audio(&sink_music, "./assets/sound/Main Menu.mp3");

    loop {
        match game.current_screen {
            crate::structs::Screens::MainMenu => {
                crate::screens::render_main_menu(&mut game, &sink_sfx);
                if sink_music.empty() {
                    rotilities::play_audio(&sink_music, "./assets/sound/Main Menu.mp3");
                }
            },
            crate::structs::Screens::SettingsMenu => {
                if sink_music.empty() {
                    rotilities::play_audio(&sink_music, "./assets/sound/Main Menu.mp3");
                }
            },
            crate::structs::Screens::InGame => {
                let mut new_volume = game.settings.mus_vol;
                while sink_music.volume() > 0.0 {
                    rotilities::set_audio_volume(&sink_music, new_volume);
                    new_volume -= 0.002;
                    macroquad::window::next_frame().await;
                }
                rotilities::stop_audio(&sink_music);
                rotilities::set_audio_volume(&sink_music, game.settings.mus_vol);
                std::thread::sleep(std::time::Duration::from_millis(1000));

                while game.current_screen == crate::structs::Screens::InGame {
                    render_game_screen(&mut game);
                    macroquad::window::next_frame().await;
                }
            },

            _ => {}
        }

        macroquad::window::next_frame().await;
    }
}
