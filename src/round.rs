use rand::Rng;
use std::io;

use crate::player;
use crate::weapons;

pub use player::Player;
pub use weapons::{weapons_map, Weapon, WeaponKind};

pub fn handler(p1: &mut Player, p2: &mut Player, game_i: i32) {
    /* Main function to handle rounds */
    println!(
        "\nRound {}!\n\nChoose your weapon:\n  - Rock\n  - Paper\n  - Scissor",
        game_i
    );
    weapon_selection(p1, p2);
    determine_outcome(p1, p2);
}

pub fn weapon_selection(p1: &mut Player, p2: &mut Player) {
    let weapons = weapons_map();
    let mut p1_choice = String::new();
    let mut p2_rng = rand::thread_rng();

    io::stdin()
        .read_line(&mut p1_choice)
        .expect("Failed to read choice.");
    p1.weapon = weapons[&p1_choice.trim().to_string().to_lowercase()];
    p2.weapon = weapons.values().cloned().collect::<Vec<Weapon>>()[p2_rng.gen_range(0..2)];
    println!("{:?}", p2.weapon);
}

pub fn determine_outcome(p1: &mut Player, p2: &mut Player) {
    if p1.weapon.beats == p2.weapon.kind {
        p1.wins += 1;
        p2.losses += 1;
    } else if p1.weapon.losses == p2.weapon.kind {
        p2.wins += 1;
        p1.losses += 1;
    } else {
        /* Draw! do nothing */
    }
}
