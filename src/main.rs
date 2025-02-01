use std::io;
use std::io::BufReader;
use std::fs::File;
mod definitions;
mod screens;
use screens::*;
mod gamedata;
use gamedata::*;
use rust_embed::Embed;
use rodio::{Decoder, OutputStream, Sink};
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

#[derive(PartialEq, Eq)]
enum Screens {
    Start,
    Settings,
    Game
}

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/sound/"]
struct Asset;

fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(BufReader::new(File::open(format!("../sound/{filename}")).expect("failed to read file")));
    return source.expect("failed to decode file")
}

fn main() -> io::Result<()> {
    let player = &mut Player {
        nickname: "player".to_string(),
        money: 0.0,
        bits: 0,
        bytes: 0,
        miners: 1,
        miner_price: 60.0,
        converters: 1,
        converter_price: 5000.0
    };

    let settings = &mut GameSettings {
        sfx_volume: 0.75,
        music_volume: 0.75,
        frame_delay: 60
    };

    let state = &mut GameState {state: "Start Game".to_string()};
    let mut current_screen = Screens::Start;
    let mut terminal = ratatui::init();
    let automate = false;
    let (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let sink_music = Sink::try_new(&stream_handle).unwrap();
    let sink_sfx = Sink::try_new(&stream_handle).unwrap();
    sink_sfx.append(get_source("interact.mp3"));
    sink_music.append(get_source("music2.mp3"));
    sink_sfx.sleep_until_end();

    loop {
        let _ = terminal.clear();

        while current_screen == Screens::Start {
            render_main_menu(&mut terminal, state.state.clone());

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                        sink_sfx.append(get_source("interact.mp3"));
                        definitions::sleep(300);
                        ratatui::restore();
                        return Ok(());
                    }

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                        sink_sfx.append(get_source("interact.mp3"));
                        current_screen = Screens::Settings;
                        break;
                    }

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                        sink_sfx.append(get_source("interact.mp3"));
                        sink_music.stop();
                        sink_music.append(get_source("music1.mp3"));
                        state.state="Back to Game".to_string();
                        current_screen = Screens::Game;
                        break;
                    }
                    else {continue}
                }
            }
        }

        while current_screen == Screens::Settings {
            render_settings_menu(&mut terminal, settings);

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                        sink_sfx.append(get_source("interact.mp3"));
                        current_screen = Screens::Start;
                        break;
                    }

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('+')  {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("interact.mp3"));
                        settings.frame_delay += 1;
                        continue;
                    }

                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('-') && settings.frame_delay>1  {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("interact.mp3"));
                        settings.frame_delay -= 1;
                        continue;
                    }
                    else {continue}
                }
            }
            definitions::sleep(10);
        }

        while current_screen == Screens::Game {
            render_game(&mut terminal, player);

            //definitions::sleep(settings.frame_delay);

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    if key.code == KeyCode::Char('q')  {
                        sink_sfx.append(get_source("interact.mp3"));
                        sink_music.stop();
                        sink_music.append(get_source("music2.mp3"));
                        current_screen = Screens::Start;
                        break;
                    }

                    if key.code == KeyCode::Char('e')  {
                        sink_sfx.append(get_source("interact.mp3"));
                        current_screen = Screens::Settings;
                        break;
                    }

                    if key.code == KeyCode::Char('1')  {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("mining.mp3"));
                        player.bits += 1*&player.miners;
                    }

                    if key.code == KeyCode::Char('2') && player.bits > 0 {
                        sink_sfx.append(get_source("sell.mp3"));
                        player.money += player.bits as f32;
                        player.bits = 0;
                    }

                    if key.code == KeyCode::Char('3') && player.bits >= 8*player.converters {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("interact.mp3"));
                        player.bytes += 1*player.converters;
                        player.bits -= 8*player.converters;
                    }

                    if key.code == KeyCode::Char('6') && player.money >= player.miner_price {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("bought.mp3"));
                        player.miners += 1;
                        player.money -= player.miner_price;
                        player.miner_price *= 1.5;
                    }

                    if key.code == KeyCode::Char('7') && player.money >= player.converter_price {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("bought.mp3"));
                        player.converters += 1;
                        player.money -= player.converter_price;
                        player.converter_price *= 1.5;
                    }
                }
            }
        }
    }
}
