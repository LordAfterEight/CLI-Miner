use std::io;
use std::io::BufReader;
use std::fs::File;
mod definitions;
mod screens;
use screens::*;
mod gamedata;
use gamedata::*;
use rodio::{Decoder, OutputStream, Sink};
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(BufReader::new(File::open(format!("../sound/{filename}")).unwrap()));
    return source.unwrap()
}

fn main() -> io::Result<()> {
    let player = &mut Player {
        nickname: "player".to_string(),
        money: 0.0,
        bits: 0,
        bytes: 0,
        miners: 1,
        miner_price: 60.0,
        converters: 0,
        converter_price: 5000.0
    };

    let state = &mut Gamestate {state: "Start Game".to_string()};
    let mut frame_delay = 50;
    let mut game_screen = false;
    let mut main_menu_screen = true;
    let mut settings_menu_screen = false;
    let mut terminal = ratatui::init();
    let mut sound_volume = 0.5;
    let mut sfx_volume = 0.5;

    let (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let sink_music = Sink::try_new(&stream_handle).unwrap();
    let sink_sfx = Sink::try_new(&stream_handle).unwrap();
    sink_sfx.append(get_source("interact.mp3"));
    sink_music.append(get_source("music2.mp3"));
    sink_sfx.sleep_until_end();

    loop {
        let _ = terminal.clear();
        while main_menu_screen==true {
            render_main_menu(&mut terminal, state.state.clone());
            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    sink_sfx.append(get_source("interact.mp3"));
                    definitions::sleep(300);
                    ratatui::restore();
                    return Ok(());
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    sink_sfx.append(get_source("interact.mp3"));
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                    sink_sfx.append(get_source("interact.mp3"));
                    sink_music.stop();
                    sink_music.append(get_source("music1.mp3"));
                    state.state="Back to Game".to_string();
                    main_menu_screen = false;
                    game_screen = true;
                    break;
                }
                else {continue}
            }
        }

        while settings_menu_screen==true {
            render_settings_menu(&mut terminal, frame_delay, sfx_volume, sound_volume);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    sink_sfx.append(get_source("interact.mp3"));
                    settings_menu_screen = false;
                    main_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('+')  {
                    sink_sfx.stop();
                    sink_sfx.append(get_source("interact.mp3"));
                    frame_delay += 1;
                    continue;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('-') && frame_delay>1  {
                    sink_sfx.stop();
                    sink_sfx.append(get_source("interact.mp3"));
                    frame_delay -= 1;
                    continue;
                }



                else {continue}
            }
            definitions::sleep(10);
        }

        while game_screen==true {
            render_game(&mut terminal, player.bits, player.bytes, player.miners, player.converters, player.miner_price, player.money);

            definitions::sleep(frame_delay);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    sink_sfx.append(get_source("interact.mp3"));
                    sink_music.stop();
                    sink_music.append(get_source("music2.mp3"));
                    game_screen = false;
                    main_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    sink_sfx.append(get_source("interact.mp3"));
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('1')  {
                    sink_sfx.stop();
                    sink_sfx.append(get_source("mining.mp3"));
                    player.bits += 1*&player.miners;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('2')  {
                    sink_sfx.append(get_source("sell.mp3"));
                    player.money += player.bits as f32;
                    player.bits = 0;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('6') && player.money >= player.miner_price {
                    sink_sfx.stop();
                    sink_sfx.append(get_source("bought.mp3"));
                    player.miners += 1;
                    player.money -= player.miner_price;
                    player.miner_price *= 1.5;
                }
            }
        }
    }
}
