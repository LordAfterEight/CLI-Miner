use std::{io,thread,time};
use crate::definitions::*;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,                                                                    DefaultTerminal,
};

pub fn main_menu(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let menu_ui = Paragraph::new("
            Build: 17.01.2025, made by Elias Stettmayer

            ┏━━━━━━━━━━━━━━━━━━━━━━ CLI-Miner ━━━━━━━━━━━━━━━━━━━━━┓
            ┃ Exit [q]                                             ┃
            ┃                                                      ┃
            ┃           Press [Enter] to start the game            ┃
            ┃                                                      ┃
            ┃           Press [e] to open settings                 ┃
            ┃                                                      ┃
            ┃                                                      ┃
            ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
            ");
            let menu_display = menu_ui
                .white()
                .on_black();
            frame.render_widget(menu_display, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e')
            {
                println!("Settings opened");
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('\n') {
                println!("Enter pressed");
                sleep(1000);
                return Ok(());
            }
        }
    }
}

pub fn settings_menu(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let menu_ui = Paragraph::new("
            Build: 17.01.2025, made by Elias Stettmayer

            ┏━━━━━━━━━━━━━━━━━━━━━━ Settings ━━━━━━━━━━━━━━━━━━━━━━┓
            ┃ Back [q]                                             ┃
            ┃                                                      ┃
            ┃           ToDo                                       ┃
            ┃                                                      ┃
            ┃           ToDo                                       ┃
            ┃                                                      ┃
            ┃                                                      ┃
            ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
            ");
            let menu_display = menu_ui
                .white()
                .on_black();
            frame.render_widget(menu_display, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }            
        }
    }
}
