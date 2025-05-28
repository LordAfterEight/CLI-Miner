use std::io;
mod definitions;
use definitions::*;
mod screens;
use screens::*;
mod gamedata;
use gamedata::*;
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};
use discord_rich_presence::{
    activity,
    activity::Assets,
    DiscordIpc, 
    DiscordIpcClient};
use clearscreen;
use colored::Colorize;
use rand;

#[cfg(not(target_arch = "aarch64"))]
use rotilities::*;


#[derive(PartialEq, Eq)]
enum Screens {
    Start,
    Settings,
    KeybindSettings,
    Game,
    DeviceManagement
}


fn main() -> io::Result<()> {

    let mut setting_position = 1;
    let mut menu_selection = 1;
    let mut keybind_selection = 1;
    let mut current_screen = Screens::Start;
    let mut prev_was_ingame = false;
    let mut key_is_selected = false;
    let mut miner_list: &mut Vec<Device> = &mut Vec::new();
    //let mut miner_list = read_gamedata(&mut miner_list);


    #[cfg(not(target_arch = "aarch64"))] {
        let (_stream,stream_handle) = init();
        let sink_music = new_sink(&stream_handle);
        let sink_sfx = new_sink(&stream_handle);
    }


    _ = clearscreen::clear();

    let keybinds = &mut Keybinds {
        back: KeyCode::Char('q'),
        enter: KeyCode::Enter,
        nav_up: KeyCode::Up,
        nav_down: KeyCode::Down,
        use_miner: KeyCode::Char(' '),
        use_converter: KeyCode::Char('4'),
        sell_bits: KeyCode::Char('2'),
        sell_bytes: KeyCode::Char('3'),
        buy_miner: KeyCode::Char('6'),
        buy_converter: KeyCode::Char('7')
    };

    println!("{}", "[i] Creating instances...\n".cyan());
    sleep(250);

    let player = &mut Player {
        nickname: "default".bold().underline().cyan().to_string(),
        money: 0.0,
        bits: 0,
        bytes: 0,
        miners: miner_list.len(),
        miner_price: 60.0,
        converters: 0,
        converter_price: 5000.0
    };
    println!("{}", "[✓] Player instance created".green());
    sleep(100);
    read_player_data(player);
    println!("{} {}", "  ┗━[i] Loaded player: ".cyan(), player.nickname);
    sleep(250);

    let bytestrings = &mut Bytestrings {
        bytestring_1: 0b0000_0000u8,
        bytestring_2: 0b0000_0000u8,
        bytestring_3: 0b0000_0000u8,
        bytestring_4: 0b0000_0000u8,
        bytestring_5: 0b0000_0000u8,
        bytestring_6: 0b0000_0000u8,
        bytestring_7: 0b0000_0000u8,
        bytestring_8: 0b0000_0000u8,
    };

    println!("{}", "[✓] Bytestrings instance created".green());
    sleep(100);


    let settings = &mut GameSettings {
        sfx_volume: 0.5,
        music_volume: 0.5,
        frame_delay: 65
    };
    println!("{}", "[✓] Settings instance created".green());
    read_settings_data(settings);
    sleep(100);

    let game = &mut GameState {
        state: "Start Game".to_string(),
        rich_presence_state: "In Main Menu".to_string(),
        progress_level: 1
    };

    println!("{}", "[✓] GameState instance created\n".green());
    sleep(100);

    println!("{}", "[i] Attempting to connect to Discord client...".cyan());
    sleep(250);

    let mut client = DiscordIpcClient::new("1335715218851893389").expect("");
    let mut client_state = false;

    if client.connect().is_ok() {
        println!("{}", "[✓] Connected successfully\n".green());
        client_state = true;
    } else {
        println!("{}", "[!] Connection failed\n".truecolor(250,125,0));
    }

    sleep(500);
    let icon = Assets::new();
    let small_image = icon.small_image("../assets/rich_presence_icon.png");

    _ = client.set_activity(activity::Activity::new()
        .state(format!("{}",game.rich_presence_state).as_str())
        .activity_type(activity::ActivityType::Playing)
        .assets(small_image)
    );

    #[cfg(not(target_arch = "aarch64"))] {
        sink_sfx.set_volume(settings.sfx_volume);
        sink_music.set_volume(settings.music_volume);
        play_audio(&sink_sfx, "../sound/interact.mp3");
        play_audio(&sink_music, "../sound/music2.mp3");
    }

    let mut os_is_android = false;
    #[cfg(target_arch = "aarch64")] {
        println!("{}", "\n[!] target architecture doesn't support audio".truecolor(250,125,0));
        os_is_android = true;
        sleep(250);
    }

    println!("{}", "\n[i] Starting game...".bold().cyan());
    sleep(500);

    let mut terminal = ratatui::init();
    loop {

        while current_screen == Screens::Start {    //MAIN MENU
            render_main_menu(
                &mut terminal,
                game.state.clone(),
                client_state,
                os_is_android,
                keybinds
            );

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    if key.code == keybinds.back  {
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/interact.mp3");
                        definitions::sleep(300);
                        ratatui::restore();
                        for i in 0..miner_list.len() {
                            println!("Miner {} ID: {}", i, miner_list[i].id);
                        }
                        return Ok(());
                    }

                    if key.code == KeyCode::Char('e')  {
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/interact.mp3");
                        current_screen = Screens::Settings;
                        break;
                    }

                    if key.code == keybinds.enter {
                        #[cfg(not(target_arch = "aarch64"))] {
                            play_audio(&sink_sfx, "../sound/interact.mp3");
                            stop_audio(&sink_music);
                            play_audio(&sink_music, "../sound/music1.mp3");
                        }
                        game.state="Back to Game".to_string();
                        current_screen = Screens::Game;
                        game.rich_presence_state = "Mining".to_string();
                        _ = client.set_activity(activity::Activity::new()
                            .state(format!("{}",game.rich_presence_state).as_str())
                        );
                        break;
                    }
                    else {continue}
                }
            }

            #[cfg(not(target_arch = "aarch64"))]
            if sink_music.len() == 0 {
                play_audio(&sink_music, "../sound/music2.mp3");
            }
        }

        while current_screen == Screens::Settings {     //SETTINGS
            render_settings_menu(
                &mut terminal,
                settings,
                setting_position,
                keybinds
            );

            #[cfg(not(target_arch = "aarch64"))] {
                sink_sfx.set_volume(settings.sfx_volume);
                sink_music.set_volume(settings.music_volume);
            }

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    #[cfg(not(target_arch = "aarch64"))] {
                        sink_sfx.stop();
                        play_audio(&sink_sfx, "../sound/interact.mp3");
                    }

                    if key.code == keybinds.back  {
                        save_settings_data(settings);
                        if prev_was_ingame {
                            current_screen = Screens::Game;
                        } else {current_screen = Screens::Start;}
                        break;
                    }

                    match key.code {
                        KeyCode::Up => setting_position -= 1,

                        KeyCode::Down => setting_position +=1,

                        KeyCode::Right => match setting_position {
                            1 => settings.frame_delay += 1,
                            2 => settings.sfx_volume += 0.05,
                            3 => settings.music_volume += 0.05,
                            _ => {
                                #[cfg(not(target_arch = "aarch64"))]
                                play_audio(&sink_sfx, "../sound/fail.mp3");
                            }
                        },

                        KeyCode::Left => match setting_position {
                            1 => match settings.frame_delay {
                                1 => {
                                    #[cfg(not(target_arch = "aarch64"))]
                                    play_audio(&sink_sfx, "../sound/fail.mp3");
                                },
                                _ => settings.frame_delay -= 1
                            },

                            2 => match settings.sfx_volume {
                                0.0 => settings.sfx_volume = 0.0,
                                _ => settings.sfx_volume -= 0.05
                            },

                            3 => match settings.music_volume {
                                0.0 => settings.music_volume -= 0.0,
                                _ => settings.music_volume -= 0.05
                            },

                            _ => {}//sink_sfx.append(get_source("fail.mp3"))
                        },


                        KeyCode::Enter => match setting_position {
                            4 => current_screen = Screens::KeybindSettings,
                            _ => {}
                        },

                        _ => {} //sink_sfx.append(get_source("fail.mp3"))
                    }

                    if settings.sfx_volume < 0.0 {
                        settings.sfx_volume = 0.0;
                    }

                    if settings.music_volume < 0.0 {
                        settings.music_volume = 0.0;
                    }

                    if setting_position == 5 {setting_position = 1;}
                    if setting_position == 0 {setting_position = 3;}
                }
            }
        }

        while current_screen == Screens::KeybindSettings {
            render_keybinds_menu(
                &mut terminal,
                keybinds,
                keybind_selection,
                key_is_selected
            );

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == keybinds.back {
                        current_screen = Screens::Settings;
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/interact.mp3");
                    }

                    if key.code == keybinds.nav_up && keybind_selection >= 1 {
                        keybind_selection -= 1;
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/fail.mp3");
                    };
                    if key.code == keybinds.nav_down {
                        keybind_selection += 1;
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/fail.mp3");
                    }
                    match keybind_selection {
                        0 => keybind_selection = 8,
                        9 => keybind_selection = 1,
                        _ => {}
                    }

                    if key.code == keybinds.enter && key_is_selected == false {
                        key_is_selected=true;
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/fail.mp3");
                    };

                    while key_is_selected {
                        render_keybinds_menu(
                            &mut terminal,
                            keybinds,
                            keybind_selection,
                            key_is_selected
                        );

                        if let event::Event::Key(input) = event::read()? {
                            if input.kind == KeyEventKind::Press {
                                match keybind_selection {
                                    1 => keybinds.back = input.code,
                                    2 => keybinds.enter = input.code,
                                    3 => keybinds.nav_up = input.code,
                                    4 => keybinds.nav_down = input.code,
                                    5 => keybinds.use_miner = input.code,
                                    6 => keybinds.use_converter = input.code,
                                    7 => keybinds.sell_bits = input.code,
                                    8 => keybinds.sell_bytes = input.code,
                                    _ => {}
                                }
                                key_is_selected = false;
                                sleep(250);
                                break;
                            }
                        }
                    }
                }
            }
        }

        #[cfg(not(target_arch = "aarch64"))]
        if sink_music.len() == 0 {
            play_audio(&sink_music, "../sound/music2.mp3");
        }

        while current_screen == Screens::Game {     //GAME
            render_game(
                &mut terminal,
                player,
                bytestrings,
                menu_selection,
                keybinds
            );

            definitions::sleep(settings.frame_delay);

            if game.progress_level == 1 {


                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == keybinds.nav_up {
                            menu_selection-=1;
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/interact.mp3");
                            continue;
                        };
                        if key.code == keybinds.nav_down && menu_selection >= 1 {
                            menu_selection += 1;
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/interact.mp3");
                            continue;
                        }
                        match menu_selection {
                            0 => menu_selection = 1,
                            4 => menu_selection = 3,
                            _ => {}
                        }

                        if key.code == keybinds.enter && menu_selection == 1  {
                            #[cfg(not(target_arch = "aarch64"))] {
                                play_audio(&sink_sfx, "../sound/interact.mp3");
                                sink_music.stop();
                                play_audio(&sink_music, "../sound/music2.mp3");
                            }
                            save_player_data(player);
                            save_gamedata(&mut miner_list);
                            prev_was_ingame = false;
                            current_screen = Screens::Start;
                            game.rich_presence_state = "In Main Menu".to_string();
                            _ = client.set_activity(activity::Activity::new()
                                .state(format!("{}",game.rich_presence_state).as_str())
                            );
                            break;
                        }

                        if key.code == keybinds.enter && menu_selection == 2 {
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/interact.mp3");
                            prev_was_ingame = true;
                            current_screen = Screens::Settings;
                            break;
                        }

                        if key.code == keybinds.enter && menu_selection == 3 {
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/interact.mp3");
                            prev_was_ingame = true;
                            current_screen = Screens::DeviceManagement;
                            break;
                        }

                        if key.code == keybinds.use_miner  {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                play_audio(&sink_sfx, "../sound/mining.mp3");
                            }
                            generate_bytes(bytestrings); //Just for testing purposes
                            player.bits += 1*&player.miners;
                            continue;
                        }

                        if key.code == keybinds.sell_bits && player.bits > 0 {
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/sell.mp3");
                            player.money += player.bits as f64;
                            player.bits = 0;
                            continue;
                        }

                        if key.code == keybinds.sell_bytes && player.bytes > 0 {
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/sell.mp3");
                            player.money += player.bytes as f64 * 10.0;
                            player.bytes = 0;
                            continue;
                        }

                        if key.code == keybinds.use_converter && player.bits >= 8*player.converters {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                play_audio(&sink_sfx, "../sound/interact.mp3");
                            }
                            player.bytes += 1*player.converters;
                            player.bits -= 8*player.converters;
                            continue;
                        }

                        if key.code == keybinds.buy_miner && player.money >= player.miner_price {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                play_audio(&sink_sfx, "../sound/bought.mp3");
                            }
                            player.miners += 1;
                            player.money -= player.miner_price;
                            player.miner_price *= 1.5;
                            miner_list.push(Device {
                                id: rand::random_range(10000..99999),
                                integrity: 100,
                                efficiency: 1
                            });
                            continue;
                        }

                        if key.code == keybinds.buy_converter && player.money >= player.converter_price {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                play_audio(&sink_sfx, "../sound/bought.mp3");
                            }
                            player.converters += 1;
                            player.money -= player.converter_price;
                            player.converter_price *= 1.5;
                            continue;
                        }

                        else {
                            #[cfg(not(target_arch = "aarch64"))]
                            play_audio(&sink_sfx, "../sound/fail.mp3");
                        }
                    }
                }

                #[cfg(not(target_arch = "aarch64"))]
                if sink_music.len() == 0 {
                    play_audio(&sink_music, "../sound/music1.mp3");
                }


            }
        }

        while current_screen == Screens::DeviceManagement {
            render_device_management(
                &mut terminal,
                player,
                &mut miner_list
            );

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == keybinds.back {
                        #[cfg(not(target_arch = "aarch64"))]
                        play_audio(&sink_sfx, "../sound/interact.mp3");
                        current_screen = Screens::Game;
                    }

                }
            }
        }
    }
}
