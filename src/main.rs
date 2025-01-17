use std::{io,thread,time};
mod definitions;
mod menus;
use definitions::*;
use menus::*;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};


fn main() -> io::Result<()> {
    let mut settings_mode = false;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = main_menu(terminal);
    while(settings_mode) {
    }
    ratatui::restore();
    app_result
}

