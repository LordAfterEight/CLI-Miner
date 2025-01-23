use std::io;
mod definitions;
mod screens;
use screens::*;
mod gamedata;
use gamedata::*;
use soloud::*;
use clearscreen;
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

fn main() -> io::Result<()> {
    let soloud = Soloud::default().unwrap();
    let mut mp3 = audio::Wav::default();
    let player = &mut Player {
        nickname: "player".to_string(),
        money: 0.0,
        bits: 0,
        bytes: 0,
        miners: 1,
        miner_price: 60.0,
        converters: 0
    };
    let state = &mut Gamestate { state: "Start Game".to_string() };
    let mut frame_delay = 15;
    let mut game_screen = false;
    let mut main_menu_screen = true;
    let mut settings_menu_screen = false;
    let mut terminal = ratatui::init();

    loop {
        let _ = terminal.clear();
        while main_menu_screen==true {
            render_main_menu(&mut terminal, state.state.clone());
            mp3.load_mem(include_bytes!("../sound/music2.mp3")).unwrap();
            soloud.play(&mp3);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    ratatui::restore();
                    return Ok(());
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                    state.state="Back to Game".to_string();
                    main_menu_screen = false;
                    game_screen = true;
                    break;
                }
                else {continue}
            }
        }

        while settings_menu_screen==true {
            render_settings_menu(&mut terminal, frame_delay);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    settings_menu_screen = false;
                    main_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('+')  {
                    frame_delay += 1;
                    continue;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('-') && frame_delay>1  {
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
                    game_screen = false;
                    main_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('1')  {
                    player.bits += 1*&player.miners;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('2')  {
                    player.money += player.bits as f32;
                    player.bits = 0;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('6') && player.money >= player.miner_price {
                    player.miners += 1;
                    player.money -= player.miner_price;
                    player.miner_price *= 1.5;
                }
            }
        }
    }
}
