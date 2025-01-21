use ratatui::{
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

pub fn main_menu(terminal: &mut DefaultTerminal) {
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
    });
}

pub fn settings_menu(terminal: &mut DefaultTerminal) {
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
    });
}

pub fn game() {
    todo!();
}
