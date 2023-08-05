use std::{fmt::format, thread, time::{self, Duration}};

use rand::Rng;

mod player;
use player::{Player};

use crate::{console::dot_loading_animation};
mod console;
mod util;

struct Settings {
    credit: i32,
    player_count: i32,
    players: Vec<Player>
}

impl Default for Settings {
    fn default() -> Self {
        Settings{
            credit: 0,
            player_count: 0,
            players: vec![]
        }
    }
}

fn settings_setup(settings: &mut Settings, resetting: bool) {
    if resetting {
        let msg = format!("number of players was set to {}, type a new value or press enter to continue: ", settings.player_count);
        let input = console::get_input(&msg);
        let old_player_count = settings.player_count;
        if !input.number.is_none() {
            settings.player_count = input.number.unwrap()
        }

        let msg = format!("initial credit was set to {}, type a new value or press enter to continue: ", settings.credit);
        let input = console::get_input(&msg);
        if !input.number.is_none() {
            settings.credit = input.number.unwrap();
        }

        for i in 0..settings.player_count {
            if i >= old_player_count {
                let player_name = console::get_text(
                    format!("player {} is new, set a name for him: ", i+1).as_str());
                settings.players.push(Player::new(player_name, settings.credit));
                continue;
            }
            let player = &mut settings.players[i as usize];
            let msg = format!("player {} has his name set to '{}', type a new name or press enter to continue: ", i+1, player.name);
            let player_name = console::get_text(&msg);
            if !player_name.trim().is_empty() {
                player.name = player_name;
            }
            player.credit = settings.credit;
        }
       
    }
    else {
        // getting data from user
        settings.player_count = console::get_number(
            "Set the number of players: ");

        settings.credit = console::get_number("Set intial credit (that each player will begin with): ");

        for i in 0..settings.player_count {
            let player_name = console::get_text(format!("Set name for Player {}: ", i+1).as_str());
            settings.players.push(Player::new(player_name, settings.credit));
        }
    }

    
    // printing out information
    console::clear();
    println!("/////////////////////////");
    println!("number of players: {}", settings.player_count);
    println!("initial credit: {}", settings.credit);
    let mut i = 0;
    for player in settings.players.iter() {
        println!("Player {}: {}", i+1, player.name);
        i += 1;
    }
    println!("/////////////////////////");

    let respone = console::yes_no("are these settings correct? ");
    if !respone {settings_setup(settings, true)}
}

fn main() {
    // print out instructions
    println!("Welcome to this simple poker cli!");
    println!("you can quit any time by typing either 'q' or 'quit'!");
    println!("have fun!");
    dot_loading_animation("loading game", "done loading!", 2);
    
    let mut settings: Settings = Default::default();
    settings_setup(&mut settings, false);

    println!("setting basic data done");
    loop {

    }

}
