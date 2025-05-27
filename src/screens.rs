use ratatui::{
    text::{Text,Line,Span},
    layout::Alignment,
    style::{Stylize,Color},
    widgets::{Paragraph,Block,BorderType,Padding},
    DefaultTerminal,
    prelude::{Constraint,Layout,Rect,Direction}
};
use crate::{
    Player,
    GameSettings,
    Bytestrings,
    Keybinds,
    Device,
    binary_to_string
};

fn draw_data(byte: u8) -> Line<'static> {
    let data = Line::from(format!("{:08b} | {}",
        byte,
        Span::from(binary_to_string(byte))
    ));
    return data
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn render_main_menu(
    terminal: &mut DefaultTerminal,
    state: String,
    client: bool,
    os_is_android: bool,
    keybinds: &mut Keybinds
) {
    let _ = terminal.draw(|frame| {

        let empty = Line::from("");

        let info  = Line::from("Build date: 27.05.2025").centered();

        //let news  = Line::from("What's new?                                             ").centered().underlined();
        let news1  = Line::from("- Refactored handling of devices on source level (WIP)").centered();
        let _news2 = Line::from(" ").centered();
        let _news3 = Line::from(" ").centered();

        let news_vec = Text::from(vec![
            news1,
            _news2,
            _news3
        ]);

        let title = Line::from("CLI-Miner »«  |  V0.2.7 Dev Build")
            .magenta()
            .centered()
            .bold()
            .underlined();

        let exit = Line::from(format!("Exit [{}]",keybinds.back))
            .centered()
            .light_red();

        let settings = Line::from("Settings [e]")
            .centered();

        let start = Line::from(format!("{} [{}]", state, keybinds.enter))
            .centered()
            .light_green();

        let mut client_message = Line::from("Not connected to Discord client")
            .centered()
            .red();

        if client {
            client_message = Line::from("Connected to Discord client")
            .centered()
            .green();
        }

        let mut os_message = Line::from("");

        if os_is_android {
            os_message = Line::from("Due to compatibility issues with Android audio playback is not working")
            .white()
            .on_red()
            .centered()
        }

        let game_ui = Text::from(vec![
            info,
            empty.clone(),
            title,
            empty.clone(),
            exit,
            settings,
            start,
            empty.clone(),
            client_message,
            empty.clone(),
            empty.clone(),
            empty,
            os_message
        ]);

        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
                .border_type(BorderType::Thick)
                .padding(Padding::vertical(1))
                .title(" Main Menu ")
                .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

        let news_block = Paragraph::new(Text::from(news_vec))
            .block(Block::bordered()
                .border_type(BorderType::Thick)
                .padding(Padding::vertical(2))
                .title(" What's new? "))
            .white()
            .bg(Color::Indexed(233));

        let news_area = centered_rect(50, 30, frame.area());

        frame.render_widget(menu_ui, frame.area());
        frame.render_widget(news_block, news_area);
    });
}

pub fn render_settings_menu(
    terminal: &mut DefaultTerminal,
    settings: &mut GameSettings,
    setting_position: u8,
    keybinds: &mut Keybinds
) {
    let _ = terminal.draw(|frame| {
        let empty = Line::from("");

        let back = Line::from(format!("Back [{}]", keybinds.back))
            .light_red()
            .centered();

        let mut frame_delay = Line::from(
            format!("Frame Delay:  {}ms [+]/[-]", settings.frame_delay))
            .centered();

        let mut sfx_volume = Line::from(
            format!("SFX Volume:   {:.2} [+]/[-]", settings.sfx_volume))
            .centered();

        let mut music_volume = Line::from(
            format!("Music Volume: {:.2} [+]/[-]", settings.music_volume))
            .centered();

        let mut keybind_menu = Line::from("Keybinds").centered();

        match setting_position {
            1 => frame_delay=frame_delay.black().on_white(),
            2 => sfx_volume=sfx_volume.black().on_white(),
            3 => music_volume=music_volume.black().on_white(),
            4 => keybind_menu=keybind_menu.black().on_white(),
            _ => ()
        }

        if settings.sfx_volume > 1.0 {sfx_volume = sfx_volume.light_red()}
        if settings.music_volume > 1.0 {music_volume = music_volume.light_red()}

        let game_ui = Text::from(vec![
            back,
            empty.clone(),
            frame_delay,
            sfx_volume,
            music_volume,
            empty,
            keybind_menu
        ]);

        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Double)
            .padding(Padding::proportional(1))
            .title(" Settings ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

        frame.render_widget(menu_ui, frame.area());
    });
}

pub fn render_keybinds_menu(
    terminal: &mut DefaultTerminal,
    keybinds: &mut Keybinds,
    keybind_selection: u8,
    is_selected: bool
) {
    let _ = terminal.draw(|frame| {
        let empty = Line::from("");

        let back = Line::from(format!("Back [{}]", keybinds.back))
            .light_red()
            .centered();


        let mut key_back = Line::from(format!("Back: [{}]", keybinds.back))
            .centered();
        let mut key_enter = Line::from(format!("Confirm: [{}]", keybinds.enter))
            .centered();
        let mut key_nav_up = Line::from(format!("Navigate up: [{}]", keybinds.nav_up))
            .centered();
        let mut key_nav_down = Line::from(format!("Navigate down: [{}]", keybinds.nav_down))
            .centered();
        let mut key_use_miner = Line::from(format!("Use Miner: [{}]", keybinds.use_miner))
            .centered();
        let mut key_use_converter = Line::from(format!("Use Converter: [{}]", keybinds.use_converter))
            .centered();
        let mut key_sell_bits = Line::from(format!("Sell Bits: [{}]", keybinds.sell_bits))
            .centered();
        let mut key_sell_bytes = Line::from(format!("Sell Bytes: [{}]", keybinds.sell_bytes))
            .centered();

        match keybind_selection {
            1 => key_back=key_back.black().on_white(),
            2 => key_enter=key_enter.black().on_white(),
            3 => key_nav_up=key_nav_up.black().on_white(),
            4 => key_nav_down=key_nav_down.black().on_white(),
            5 => key_use_miner=key_use_miner.black().on_white(),
            6 => key_use_converter=key_use_converter.black().on_white(),
            7 => key_sell_bits=key_sell_bits.black().on_white(),
            8 => key_sell_bytes=key_sell_bytes.black().on_white(),
            _ => {}
        }

        match is_selected {
            true => match keybind_selection {
                1 => key_back=key_back.black().on_cyan(),
                2 => key_enter=key_enter.black().on_cyan(),
                3 => key_nav_up=key_nav_up.black().on_cyan(),
                4 => key_nav_down=key_nav_down.black().on_cyan(),
                5 => key_use_miner=key_use_miner.black().on_white(),
                6 => key_use_converter=key_use_converter.black().on_white(),
                7 => key_sell_bits=key_sell_bits.black().on_white(),
                8 => key_sell_bytes=key_sell_bytes.black().on_white(),
                _ => {}
            },
            _ => {}
        }

        let game_ui = Text::from(vec![
            back,
            empty.clone(),
            key_back,
            key_enter,
            key_nav_up,
            key_nav_down,
            key_use_miner,
            key_use_converter,
            key_sell_bits,
            key_sell_bytes
        ]);

        let menu_ui = Paragraph::new(Text::from(game_ui))
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Settings >> Keybinds ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

        frame.render_widget(menu_ui, frame.area());
    });
}


pub fn render_game(
    terminal: &mut DefaultTerminal,
    player: &mut Player,
    bytestrings: &mut Bytestrings,
    menu_selection: u8,
    keybinds: &mut Keybinds
) {
    let _ = terminal.draw(|frame| {
        let nav_info = Line::from("Navigate with arrow keys");
        let empty = Line::from("");
        let mut mainmenu = Line::from(format!("Main menu"))
            .light_red();
        let mut settings = Line::from(format!("Settings "));
        let mut management = Line::from(format!("Device Management"));

        match menu_selection {
            1 => mainmenu = mainmenu.black().on_light_red(),
            2 => settings = settings.black().on_white(),
            3 => management = management.black().on_white(),
            _ => {}
        }

        let menus = Text::from(vec![
            nav_info,
            empty,
            mainmenu,
            settings,
            management
        ]);

        let money = Line::from(format!("Money: {:.2}»«", player.money));
        let bits = Line::from(format!("Bits:  {}  |  [{}] to mine, [{}] to convert to »«",
            player.bits,
            keybinds.use_miner,
            keybinds.sell_bits
        ));
        let bytes = Line::from(format!("Bytes: {}  |  [{}] to convert from Bits, [{}] to convert to »«",
            player.bytes,
            keybinds.use_converter,
            keybinds.sell_bytes
        ));
        let resources = Text::from(vec![money, bits, bytes]);

        let miners = Line::from(format!("Miners: {}      |  Price: {:.2}»«, [{}] to buy", player.miners, player.miner_price, keybinds.buy_miner));
        let converters = Line::from(format!("Converters: {}  |  Price: {:.2}»«, [{}] to use, [{}] to buy",
            player.converters,
            player.converter_price,
            keybinds.use_converter,
            keybinds.buy_converter
        ));
        let devices = Text::from(vec![miners,converters]);

        let menus_ui = Paragraph::new(Text::from(menus))
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Menus ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

        let resources_ui = Paragraph::new(resources)
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Resources ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

        let devices_ui = Paragraph::new(devices)
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Devices ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));

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
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Bytes | Data Strings ")
            .title_alignment(Alignment::Center))
            .light_green()
            .bg(Color::Indexed(232));

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
        let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3]).areas(left);

        frame.render_widget(menus_ui, top);
        frame.render_widget(resources_ui, middle);
        frame.render_widget(devices_ui, bottom);
        frame.render_widget(data_ui, right);
    });
}


pub fn render_device_management(
    terminal: &mut DefaultTerminal,
    _player: &mut Player,
    miner_list: &mut Vec<Device>
) {
    let _ = terminal.draw(|frame| {

        let mut devices: Vec<Line> = Vec::new();
        for i in 0..miner_list.len() {
            devices.push(Line::from(format!("Miner {}  |  ID={}", i, miner_list[i].id)));
        };

        let devices_ui = Paragraph::new(devices)
            .block(Block::bordered()
            .border_type(BorderType::Thick)
            .padding(Padding::proportional(1))
            .title(" Device Management ")
            .title_alignment(Alignment::Center))
            .white()
            .bg(Color::Indexed(232));


        frame.render_widget(devices_ui, frame.area());
    });
}
