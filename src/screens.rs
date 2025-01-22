use ratatui::{
    text::Text,
    layout::Alignment,
    style::{Stylize},
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
};

pub fn render_main_menu(terminal: &mut DefaultTerminal) {
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

pub fn render_settings_menu(terminal: &mut DefaultTerminal) {
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

pub fn render_game(terminal: &mut DefaultTerminal, bits: u8, bytes: u8, miners: u8, converters: u8) {
    terminal.draw(|frame| {
        let game_ui = format!("Main Menu [q]\nSettings [e]\n\nBits:{bits}\n\nMiners:{miners}");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title("CLI Miner")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    }); 
}
