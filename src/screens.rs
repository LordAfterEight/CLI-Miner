use ratatui::{
    text::Text,
    layout::Alignment,
    style::Stylize,
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
    prelude::{Constraint,Layout}
};
use crate::Player;
use crate::GameSettings;

pub fn render_main_menu(terminal: &mut DefaultTerminal, state: String) {
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Exit [q]\n\nSettings [e]\n\n{state} [Enter]");
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title(" Main Menu ")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_settings_menu(terminal: &mut DefaultTerminal, settings: &mut GameSettings) {
    let _ = terminal.draw(|frame| {
        let game_ui = format!("Back [q]\n\nFrame Delay: {} [+]/[-]   Lower this setting to make the game faster or vice versa | 50-60 recommended\nWarning: There is a known bug that will cause keypresses to be buffered if the delay is too high!\n\nSFX Volume: {} [+]/[-]", settings.frame_delay, settings.sfx_volume);
        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Double)
            .padding(Padding{left:2,right:2,top:1,bottom:1})
            .title(" Settings ")
            .title_alignment(Alignment::Center));
        let menu_display = menu_ui
            .white()
            .on_black();
        frame.render_widget(menu_display, frame.area());
    });
}

pub fn render_game(terminal: &mut DefaultTerminal, player: &mut Player) {
    let _ = terminal.draw(|frame| {
        let menus = format!("Main Menu [q]\nSettings [e]\n\n");
        let resources = format!("Money: {}$\n\nBits: {}  |  [1] to mine, [2] to convert to money\nBytes: {}", player.money, player.bits, player.bytes);
        let devices = format!("Miners: {}  |  Price: {}$, [6] to buy\nConverters: {}  |  Price: {}$, [3] to use, [7] to buy", player.miners, player.miner_price, player.converters, player.converter_price);

        let menus_ui = Paragraph::new(Text::from(menus))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Menus ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        let resources_ui = Paragraph::new(Text::from(resources))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Resources ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        let devices_ui = Paragraph::new(Text::from(devices))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Devices ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        let todo_ui = Paragraph::new("")
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Todo ")
            .title_alignment(Alignment::Center))
            .light_red()
            .on_black();

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
        let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3]).areas(left);
        frame.render_widget(menus_ui, top);
        frame.render_widget(resources_ui, middle);
        frame.render_widget(devices_ui, bottom);
        frame.render_widget(todo_ui, right);
    });
}
