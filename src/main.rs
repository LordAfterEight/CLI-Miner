use std::io;
mod definitions;
mod menus;
use menus::*;
use clearscreen;

use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

fn main() -> io::Result<()> {
    let game_screen = false;
    let main_menu_screen = true;
    let settings_menu_screen = false;
    let mut terminal = ratatui::init();
    let app_result = main_menu(&mut terminal);
    clearscreen::clear();

    while main_menu_screen==true {
        main_menu(&mut terminal);

        if let event::Event::Key(key) = event::read()? {

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
                ratatui::restore();
                clearscreen::clear();
                return Ok(());
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
                clearscreen::clear();
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('\n') {
            }
            else {continue}

        }
        continue;
    }
    while game_screen==true {
        println!("
            Bits: 
            
            Miners:
        ");
        definitions::sleep(10);
        clearscreen::clear();
    } Ok(())
}

