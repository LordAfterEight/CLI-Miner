use ratatui::{
    text::{Text,Line},
    layout::Alignment,
    style::Stylize,
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
    prelude::{Constraint,Layout}
};
use crate::Player;
use crate::GameSettings;

pub fn render_main_menu(terminal: &mut DefaultTerminal, state: String, client: bool) {
    let _ = terminal.draw(|frame| {

        let empty = Line::from("");

        let title = Line::from("CLI-Miner »«  |  V0.1.0 Dev Build for Android").magenta().centered().bold().underlined();
        let exit = Line::from("Exit [q]").centered().light_red();
        let settings = Line::from("Settings [e]").centered();
        let start = Line::from(format!("{state} [Enter]")).centered().light_green();
        let disclaimer = Line::from("This is a test build only for trying to implement audio on android").centered();
        let disclaimer2 = Line::from("The newest features are not included").centered();
        let mut client_message = Line::from("Not connected to Discord client").centered().red();
        if client == true {client_message = Line::from("Connected to Discord client").centered().green();}
        let game_ui = Text::from(vec![title, empty.clone(), exit, settings, start, empty.clone(), client_message, empty, disclaimer, disclaimer2]);

        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::vertical(5))
            .title(" Main Menu ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        frame.render_widget(menu_ui, frame.area());
    });
}

pub fn render_settings_menu(terminal: &mut DefaultTerminal, settings: &mut GameSettings, mut setting_position: u8) {
    let _ = terminal.draw(|frame| {
        let empty = Line::from("");
        let back = Line::from("Back [q]").light_red().centered();
        let mut frame_delay = Line::from(format!("Frame Delay: {} [+]/[-]", settings.frame_delay)).centered();
        let mut sfx_volume = Line::from(format!("SFX Volume:   {:.2} [+]/[-]", settings.sfx_volume)).centered();
        let mut music_volume = Line::from(format!("Music Volume: {:.2} [+]/[-]", settings.music_volume)).centered();

        match setting_position {
            1 => frame_delay=frame_delay.black().on_white(),
            2 => sfx_volume=sfx_volume.black().on_white(),
            3 => music_volume=music_volume.black().on_white(),
            _ => ()
        }

        let game_ui = Text::from(vec![back, empty.clone(), frame_delay, sfx_volume, music_volume]);

        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Double)
            .padding(Padding::proportional(1))
            .title(" Settings ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        frame.render_widget(menu_ui, frame.area());
    });
}

pub fn render_game(terminal: &mut DefaultTerminal, player: &mut Player) {
    let _ = terminal.draw(|frame| {
        let mainmenu = Line::from(format!("Main menu [q]")).light_red();
        let settings = Line::from(format!("Settings [e]"));
        let menus = Text::from(vec![mainmenu, settings]);

        let money = Line::from(format!("Money: {}»«", player.money));
        let bits = Line::from(format!("Bits:  {}  |  [1] to mine, [2] to convert to »«", player.bits));
        let bytes = Line::from(format!("Bytes: {}  |  [4] to convert from Bits, [3] to convert to »«", player.bytes));
        let resources = Text::from(vec![money, bits, bytes]);

        let miners = Line::from(format!("Miners: {}      |  Price: {}»«, [6] to buy", player.miners, player.miner_price));
        let converters = Line::from(format!("Converters: {}  |  Price: {}»«, [3] to use, [7] to buy", player.converters, player.converter_price));
        let devices = Text::from(vec![miners,converters]);

        let menus_ui = Paragraph::new(Text::from(menus))
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Menus ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        let resources_ui = Paragraph::new(resources)
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Resources ")
            .title_alignment(Alignment::Center))
            .white()
            .on_black();

        let devices_ui = Paragraph::new(devices)
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
