use macroquad::prelude::*;
use macroquad::rand;
use rotilities;

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
    let (_stream, stream_handle) = rotilities::init();
    let sink_music = rotilities::new_sink(&stream_handle);

    let mut game = crate::structs::Game::init("LordAfterEight");

    println!("Game: {:#?}", game);

    rotilities::play_audio(&sink_music, "./sound/Main Menu.mp3");

    loop {
        crate::screens::render_main_menu();
        macroquad::window::next_frame().await;
    }
}
