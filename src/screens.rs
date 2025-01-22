use ratatui::{
    text::Text,
    layout::Alignment,
    style::{Stylize},
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
};

pub fn render_main_menu(terminal: &mut DefaultTerminal) {
    let _ = terminal.clear();
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Exit [q]\n\nSettings [e]\n\nStart Game [Enter]");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title("Main Menu")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_settings_menu(terminal: &mut DefaultTerminal) {
    let _ = terminal.clear();
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Back [q]");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title("Settings")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_game(terminal: &mut DefaultTerminal, bits: u8, bytes: u8, miners: u8, converters: u8) {
    let _ = terminal.clear();
    let _ = terminal.draw(|frame| {
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
