use std::io;
mod definitions;
mod screens;
use screens::*;
mod gamedata;
use gamedata::*;
use clearscreen;
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

fn main() -> io::Result<()> {
    let player = &mut Player {
        nickname: "player".to_string(),
        bits: 0,
        bytes: 0,
        miners: 1,
        converters: 0
    };
    let mut game_screen = false;
    let mut main_menu_screen = true;
    let mut settings_menu_screen = false;
    let mut terminal = ratatui::init();
    clearscreen::clear();
    
    while true {
        while main_menu_screen==true {
            main_menu(&mut terminal);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    ratatui::restore();
                    clearscreen::clear();
                    return Ok(());
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    clearscreen::clear(); 
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                    main_menu_screen = false;
                    game_screen = true;
                    clearscreen::clear();
                }
                else {continue}

            }
            continue;
        }

        while settings_menu_screen==true {
            settings_menu(&mut terminal);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    settings_menu_screen = false;
                    main_menu_screen = true;
                    clearscreen::clear();
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    clearscreen::clear(); 
                }
                else {continue}
            }
            continue;
        }

        while game_screen==true {
            game(&mut terminal, player.bits, player.bytes, player.miners, player.converters);

            if let event::Event::Key(key) = event::read()? {

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                    game_screen = false;
                    main_menu_screen = true;
                    clearscreen::clear();
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                    main_menu_screen = false;
                    settings_menu_screen = true;
                    clearscreen::clear(); 
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('1')  {
                    player.bits += 1*&player.miners;
                    clearscreen::clear();
                    continue;
                }
            }

            definitions::sleep(10); //code runs at 100Hz
            clearscreen::clear();
        }
    }
    Ok(())
}
