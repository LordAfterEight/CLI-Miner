use ratatui::{
    text::{Text,Line,Span},
    layout::Alignment,
    style::Stylize,
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
    prelude::{Constraint,Layout}
};
use crate::{
    Player,
    GameSettings,
    Bytestrings,
    sleep,
    binary_to_string
};

fn draw_data(byte: u8) -> Line<'static> {
    let data = Line::from(format!("{:08b} | {}",
        byte,
        Span::from(binary_to_string(byte))
    ));
    return data
}

pub fn render_main_menu(
    terminal: &mut DefaultTerminal,
    state: String,
    client: bool,
    os_is_android: bool
) {
    let _ = terminal.draw(|frame| {

        let empty = Line::from("");

        let title = Line::from("CLI-Miner »«  |  V0.1.4 Dev Build")
            .magenta()
            .centered()
            .bold()
            .underlined();

        let exit = Line::from("Exit [q]")
            .centered()
            .light_red();

        let settings = Line::from("Settings [e]")
            .centered();

        let start = Line::from(format!("{state} [Enter]"))
            .centered()
            .light_green();

        let mut client_message = Line::from("Not connected to Discord client")
            .centered()
            .red();

        if client == true {
            client_message = Line::from("Connected to Discord client")
            .centered()
            .green();
        }

        let mut os_message = Line::from("");

        if os_is_android == true {
            os_message = Line::from("Due to compatibility issues with Android audio playback is not working yet")
            .white()
            .on_red()
            .centered()
        }

        let game_ui = Text::from(vec![
            title,
            empty.clone(),
            exit,
            settings,
            start,
            empty.clone(),
            client_message,
            empty,
            os_message
        ]);

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

pub fn render_settings_menu(
    terminal: &mut DefaultTerminal,
    settings: &mut GameSettings,
    mut setting_position: u8
) {
    let _ = terminal.draw(|frame| {
        let empty = Line::from("");

        let back = Line::from("Back [q]")
            .light_red()
            .centered();

        let mut frame_delay = Line::from(
            format!("Frame Delay: {} [+]/[-]", settings.frame_delay))
            .centered();

        let mut sfx_volume = Line::from(
            format!("SFX Volume:   {:.2} [+]/[-]", settings.sfx_volume))
            .centered();

        let mut music_volume = Line::from(
            format!("Music Volume: {:.2} [+]/[-]", settings.music_volume))
            .centered();

        match setting_position {
            1 => frame_delay=frame_delay.black().on_white(),
            2 => sfx_volume=sfx_volume.black().on_white(),
            3 => music_volume=music_volume.black().on_white(),
            _ => ()
        }

        let game_ui = Text::from(vec![
            back,
            empty.clone(),
            frame_delay,
            sfx_volume,
            music_volume
        ]);

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

pub fn render_game(
    terminal: &mut DefaultTerminal,
    player: &mut Player,
    bytestrings: &mut Bytestrings
) {
    let _ = terminal.draw(|frame| {
        let mainmenu = Line::from(format!("Main menu [q]"))
            .light_red();

        let settings = Line::from(format!("Settings [e]"));

        let menus = Text::from(vec![mainmenu, settings]);

        let money = Line::from(format!("Money: {}»«", player.money));
        let bits = Line::from(format!("Bits:  {}  |  [Space] to mine, [2] to convert to »«", player.bits));
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

        let bytestring1 = draw_data(bytestrings.bytestring_1);
        let bytestring2 = draw_data(bytestrings.bytestring_2);
        let bytestring3 = draw_data(bytestrings.bytestring_3);
        let bytestring4 = draw_data(bytestrings.bytestring_4);
        let bytestring5 = draw_data(bytestrings.bytestring_5);
        let bytestring6 = draw_data(bytestrings.bytestring_6);
        let bytestring7 = draw_data(bytestrings.bytestring_7);
        let bytestring8 = draw_data(bytestrings.bytestring_8);

        let data = Text::from(vec![
            bytestring1,
            bytestring2,
            bytestring3,
            bytestring4,
            bytestring5,
            bytestring6,
            bytestring7,
            bytestring8
        ]);

        let data_ui = Paragraph::new(data)
            .block(Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1))
            .title(" Bytes | Data Strings ")
            .title_alignment(Alignment::Center))
            .light_green()
            .on_black();

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
        let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3]).areas(left);

        frame.render_widget(menus_ui, top);
        frame.render_widget(resources_ui, middle);
        frame.render_widget(devices_ui, bottom);
        frame.render_widget(data_ui, right);
    });
}
