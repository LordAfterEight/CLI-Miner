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
            .title(" Main Menu ")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_settings_menu(terminal: &mut DefaultTerminal, frame_delay: u64) {
    let _ = terminal.clear();
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Back [q]\n\nFrame Delay: {frame_delay} [+]/[-]   Lower this setting to make the game faster or vice versa | 10-20 recommended
                           Warning: There is a known bug that will cause keypresses to be buffered if the delay is too high!");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title(" Settings ")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_game(terminal: &mut DefaultTerminal, bits: u32, bytes: u32, miners: u32, converters: u32, miner_price: f32, money: f32) {
    let _ = terminal.clear();
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Main Menu [q]\nSettings [e]\n\nMoney: {money}$\n\nBits: {bits}    [2] to convert to money\n\nMiners: {miners}  Price: {miner_price}, [6] to buy");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title(" CLI-Miner ")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}
