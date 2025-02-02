use ratatui::{
    text::{Text,Line,Span},
    layout::Alignment,
    style::{Stylize,Style},
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
    prelude::{Constraint,Layout}
};
use crate::Player;
use crate::GameSettings;

pub fn render_main_menu(terminal: &mut DefaultTerminal, state: String) {
    let _ = terminal.draw(|frame| {

        let empty_line = Line::from("");

        let title = Line::from("CLI-Miner »«").magenta().centered().bold().underlined();
        let exit = Line::from("Exit [q]").centered().light_red();
        let settings = Line::from("Settings [e]").centered();
        let start = Line::from(format!("{state} [Enter]")).centered().light_green();
        let game_ui = Text::from(vec![title, empty_line, exit, settings, start]);

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

pub fn render_settings_menu(terminal: &mut DefaultTerminal, settings: &mut GameSettings) {
    let _ = terminal.draw(|frame| {
        let back = Line::from("Back [q]").light_red();
        let frame_delay_setting = Line::from(format!("Frame Delay: {} [+]/[-]", settings.frame_delay));
        let sfx_volume_setting = Line::from(format!("SFX Volume: {} [+]/[-]", settings.sfx_volume));
        let game_ui = Text::from(vec![back, frame_delay_setting, sfx_volume_setting]);

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
        let bits = Line::from(format!("Bits:  {}", player.bits));
        let bytes = Line::from(format!("Bytes: {}", player.bytes));
        let bitsinfo  = Line::from("|  [1] to mine, [2] to convert to »«").right_aligned();
        let bytesinfo = Line::from("|  [3] to convert from Bits").right_aligned();
        let resources = Text::from(vec![money, bits, bytes, bitsinfo, bytesinfo]);

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
