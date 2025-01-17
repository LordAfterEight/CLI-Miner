use std::{io,thread,time};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = main_menu(terminal);
    ratatui::restore();
    app_result
}

fn main_menu(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let menu_ui = Paragraph::new("
            ┏━━━━━━━━━━━━━━━━━━━━━━ CLI-Miner ━━━━━━━━━━━━━━━━━━━━━┓
            ┃                                                      ┃             
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

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') {
                println!("Settings opened");
                thread::sleep(time::Duration::from_millis(1000));
                return Ok(());
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('\n') {
                println!("Enter pressed");
                thread::sleep(time::Duration::from_millis(1000));
                return Ok(());
            }
        }
    }
}
