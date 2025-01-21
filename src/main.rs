use std::io;
mod definitions;
mod menus;
use menus::*;
use definitions::*;
use std::fs::File;
use clearscreen;

use ratatui::{crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal,
    style::Stylize,
    widgets::Paragraph,
};

fn main() -> io::Result<()> {
    let mut settings_mode = false;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = main_menu(terminal);
    ratatui::restore();
    app_result;

    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')  {
            return Ok(());
        }
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')  {
        }
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('\n') {
        }

    }

    while(true) {
        println!("
            Bits: 
            
            Miners:
        ");
        clearscreen::clear();
    } Ok(())
}

