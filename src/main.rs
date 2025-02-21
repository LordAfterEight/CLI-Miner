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
use rand;
use clearscreen;
use colored::Colorize;

#[cfg(not(target_arch = "aarch64"))]
use rodio::{OutputStream, Sink};


#[derive(PartialEq, Eq)]
enum Screens {
    Start,
    Settings,
    Game
}


fn main() -> io::Result<()> {
    clearscreen::clear();

    println!("{}", "[i] Creating objects...\n".blue());
    sleep(250);

    let player = &mut Player {
        nickname: "default".bold().underline().cyan().to_string(),
        money: 0.0,
        bits: 0,
        bytes: 0,
        miners: 1,
        miner_price: 60.0,
        converters: 0,
        converter_price: 5000.0
    };
    println!("{}", "[✓] Player object created".green());
    sleep(100);
    read_data(player);
    println!("{} {}", "  ┗━[i] Loaded player: ".blue(), player.nickname);
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

    println!("{}", "[✓] Bytestrings object created".green());
    sleep(100);


    let settings = &mut GameSettings {
        sfx_volume: 0.5,
        music_volume: 0.5,
        frame_delay: 65
    };
    println!("{}", "[✓] Settings object created".green());
    sleep(100);

    let game = &mut GameState {
        state: "Start Game".to_string(),
        rich_presence_state: "In Main Menu".to_string(),
        progress_level: 1
    };

    println!("{}", "[✓] Game object created\n".green());
    sleep(100);

    println!("{}", "[i] Attempting to connect to Discord client...".blue());
    sleep(250);

    let mut client = DiscordIpcClient::new("1335715218851893389").expect("");
    if client.connect().is_ok() {
        println!("{}", "[✓] Connected successfully\n".green());
    }
    if client.connect().is_err() {
        println!("{}", "[!] Connection failed\n".truecolor(200,100,0));
    }
    let client_state = client.connect().is_ok();

    sleep(500);
    let icon = Assets::new();
    let small_image = icon.small_image("../assets/rich_presence_icon.png");

    _ = client.set_activity(activity::Activity::new()
        .state(format!("{}",game.rich_presence_state).as_str())
        .activity_type(activity::ActivityType::Playing)
        .assets(small_image)
    );

    let mut setting_position = 1;
    let mut menu_selection = 1;
    let mut current_screen = Screens::Start;
    let mut prev_was_ingame = false;

    /*
    let (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let sink_music = Sink::try_new(&stream_handle).unwrap();
    let sink_sfx = Sink::try_new(&stream_handle).unwrap();

    sink_sfx.set_volume(settings.sfx_volume);
    sink_music.set_volume(settings.music_volume);

    sink_sfx.append(get_source("interact.mp3"));
    sink_music.append(get_source("music2.mp3"));
    sink_sfx.sleep_until_end();*/


    let mut os_is_android = false;
    #[cfg(target_arch = "aarch64")] {
        println!("{}", "\n[!] target architecture doesn't support audio".truecolor(200,100,0));
        os_is_android = true;
        sleep(250);
    }

    println!("{}", "\n[i] Starting game...".bold().cyan());
    sleep(500);

    let mut terminal = ratatui::init();
    loop {

        while current_screen == Screens::Start {
            render_main_menu(
                &mut terminal,
                game.state.clone(), 
                client_state, 
                os_is_android
            );

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    if key.code == KeyCode::Char('q')  {
                        #[cfg(not(target_arch = "aarch64"))]
                        sink_sfx.append(get_source("interact.mp3"));
                        definitions::sleep(300);
                        ratatui::restore();
                        return Ok(());
                    }

                    if key.code == KeyCode::Char('e')  {
                        #[cfg(not(target_arch = "aarch64"))]
                        sink_sfx.append(get_source("interact.mp3"));
                        current_screen = Screens::Settings;
                        break;
                    }

                    if key.code == KeyCode::Enter {
                        #[cfg(not(target_arch = "aarch64"))] {
                            sink_sfx.append(get_source("interact.mp3"));
                            sink_music.stop();
                            sink_music.append(get_source("music1.mp3"));
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
                sink_music.append(get_source("music2.mp3"));
            }
        }

        while current_screen == Screens::Settings {
            render_settings_menu(
                &mut terminal,
                settings,
                setting_position,
            );

            #[cfg(not(target_arch = "aarch64"))] {
                sink_sfx.set_volume(settings.sfx_volume);
                sink_music.set_volume(settings.music_volume);
            }

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    #[cfg(not(target_arch = "aarch64"))] {
                        sink_sfx.stop();
                        sink_sfx.append(get_source("interact.mp3"));
                    }

                    if key.code == KeyCode::Char('q')  {
                        if prev_was_ingame {
                            current_screen = Screens::Game;
                        } else {current_screen = Screens::Start;}
                        break;
                    }

                    match key.code {
                        KeyCode::Up => setting_position -= 1,
                        KeyCode::Down => setting_position +=1,
                        KeyCode::Char('+') => match setting_position {
                            1 => settings.frame_delay += 1,
                            2 => settings.sfx_volume += 0.05,
                            3 => settings.music_volume += 0.05,
                            _ => {} //sink_sfx.append(get_source("fail.mp3"))
                        },
                        KeyCode::Char('-') => match setting_position {
                            1 => match settings.frame_delay {
                                1 => {}, //sink_sfx.append(get_source("fail.mp3")),
                                _ => settings.frame_delay -= 1},
                            2 => match settings.sfx_volume {
                                0.0 => settings.sfx_volume = 0.0,
                                _ => settings.sfx_volume -= 0.05},
                            3 => match settings.music_volume {
                                0.0 => settings.music_volume -= 0.0,
                                _ => settings.music_volume -= 0.05},
                            _ => {}}, //sink_sfx.append(get_source("fail.mp3"))},
                        _ => {} //sink_sfx.append(get_source("fail.mp3"))
                    }
                    if setting_position == 4 {setting_position = 1;}
                    if setting_position == 0 {setting_position = 3;}
                }
            }

            #[cfg(not(target_arch = "aarch64"))]
            if sink_music.len() == 0 {
                sink_music.append(get_source("music2.mp3"));
            }
        }

        while current_screen == Screens::Game {
            render_game(&mut terminal, player, bytestrings, menu_selection);

            definitions::sleep(settings.frame_delay);

            if game.progress_level == 1 {


                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Esc {break;}

                        if key.code == KeyCode::Up {menu_selection-=1};
                        if key.code == KeyCode::Down && menu_selection >= 1 {
                            menu_selection += 1;
                        }
                        match menu_selection {
                            0 => menu_selection = 1,
                            3 => menu_selection = 2,
                            _ => {}
                        }

                        if key.code == KeyCode::Enter && menu_selection == 1  {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.append(get_source("interact.mp3"));
                                sink_music.stop();
                                sink_music.append(get_source("music2.mp3"));
                            }
                            save_data(player);
                            prev_was_ingame = false;
                            current_screen = Screens::Start;
                            game.rich_presence_state = "In Main Menu".to_string();
                            _ = client.set_activity(activity::Activity::new()
                                .state(format!("{}",game.rich_presence_state).as_str())
                            );
                            break;
                        }

                        if key.code == KeyCode::Enter && menu_selection == 2 {
                            #[cfg(not(target_arch = "aarch64"))]
                            sink_sfx.append(get_source("interact.mp3"));
                            prev_was_ingame = true;
                            current_screen = Screens::Settings;
                            break;
                        }

                        if key.code == KeyCode::Char(' ')  {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                sink_sfx.append(get_source("mining.mp3"));
                            }
                            generate_bytes(bytestrings); //Just for testing purposes
                            player.bits += 1*&player.miners;
                            continue;
                        }

                        if key.code == KeyCode::Char('2') && player.bits > 0 {
                            #[cfg(not(target_arch = "aarch64"))]
                            sink_sfx.append(get_source("sell.mp3"));
                            player.money += player.bits as f64;
                            player.bits = 0;
                            continue;
                        }

                        if key.code == KeyCode::Char('3') && player.bytes > 0 {
                            #[cfg(not(target_arch = "aarch64"))]
                            sink_sfx.append(get_source("sell.mp3"));
                            player.money += player.bytes as f64 * 10.0;
                            player.bytes = 0;
                            continue;
                        }

                        if key.code == KeyCode::Char('4') && player.bits >= 8*player.converters {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                sink_sfx.append(get_source("interact.mp3"));
                            }
                            player.bytes += 1*player.converters;
                            player.bits -= 8*player.converters;
                            continue;
                        }

                        if key.code == KeyCode::Char('6') && player.money >= player.miner_price {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                sink_sfx.append(get_source("bought.mp3"));
                            }
                            player.miners += 1;
                            player.money -= player.miner_price;
                            player.miner_price *= 1.5;
                            continue;
                        }

                        if key.code == KeyCode::Char('7') && player.money >= player.converter_price {
                            #[cfg(not(target_arch = "aarch64"))] {
                                sink_sfx.stop();
                                sink_sfx.append(get_source("bought.mp3"));
                            }
                            player.converters += 1;
                            player.money -= player.converter_price;
                            player.converter_price *= 1.5;
                            continue;
                        }

                        else {
                            #[cfg(not(target_arch = "aarch64"))]
                            sink_sfx.append(get_source("fail.mp3"));
                        }
                    }
                }

                #[cfg(not(target_arch = "aarch64"))]
                if sink_music.len() == 0 {
                    sink_music.append(get_source("music1.mp3"));
                }


            }
        }
    }
}
