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

#[cfg(test)]
mod testing_determine_outcome_logic {
    use super::determine_outcome;
    use crate::weapons_map;
    use crate::Player;

    #[test]
    fn rock_beats_scissor() {
        let weapons = weapons_map();
        let mut rock_player = Player {
            name: String::from("expected_winner"),
            weapon: weapons[&String::from("rock")],
            ..Default::default()
        };
        let mut scissor_player = Player {
            name: String::from("expected_loser"),
            weapon: weapons[&String::from("scissor")],
            ..Default::default()
        };
        determine_outcome(&mut rock_player, &mut scissor_player);
        assert_eq!(1, rock_player.wins);
        assert_eq!(0, rock_player.losses);
        assert_eq!(0, scissor_player.wins);
        assert_eq!(1, scissor_player.losses);
    }

    #[test]
    fn scissor_beats_paper() {
        let weapons = weapons_map();
        let mut scissor_player = Player {
            name: String::from("expected_winner"),
            weapon: weapons[&String::from("scissor")],
            ..Default::default()
        };
        let mut paper_player = Player {
            name: String::from("expected_loser"),
            weapon: weapons[&String::from("paper")],
            ..Default::default()
        };
        determine_outcome(&mut scissor_player, &mut paper_player);
        assert_eq!(1, scissor_player.wins);
        assert_eq!(0, scissor_player.losses);
        assert_eq!(0, paper_player.wins);
        assert_eq!(1, paper_player.losses);
    }

    #[test]
    fn paper_beats_rock() {
        let weapons = weapons_map();
        let mut paper_player = Player {
            name: String::from("expected_winner"),
            weapon: weapons[&String::from("paper")],
            ..Default::default()
        };
        let mut rock_player = Player {
            name: String::from("expected_loser"),
            weapon: weapons[&String::from("rock")],
            ..Default::default()
        };
        determine_outcome(&mut paper_player, &mut rock_player);
        assert_eq!(1, paper_player.wins);
        assert_eq!(0, paper_player.losses);
        assert_eq!(0, rock_player.wins);
        assert_eq!(1, rock_player.losses);
    }

    #[test]
    fn rock_draws_rock() {
        let weapons = weapons_map();
        let mut rock_player_1 = Player {
            name: String::from("Rock Player 1"),
            weapon: weapons[&String::from("rock")],
            ..Default::default()
        };
        let mut rock_player_2 = Player {
            name: String::from("Rock Player 2"),
            weapon: weapons[&String::from("rock")],
            ..Default::default()
        };
        determine_outcome(&mut rock_player_1, &mut rock_player_2);
        assert_eq!(0, rock_player_1.wins);
        assert_eq!(0, rock_player_1.losses);
        assert_eq!(0, rock_player_2.wins);
        assert_eq!(0, rock_player_2.losses);
    }

    #[test]
    fn paper_draws_paper() {
        let weapons = weapons_map();
        let mut paper_player_1 = Player {
            name: String::from("Paper Player 1"),
            weapon: weapons[&String::from("paper")],
            ..Default::default()
        };
        let mut paper_player_2 = Player {
            name: String::from("Paper Player 2"),
            weapon: weapons[&String::from("paper")],
            ..Default::default()
        };
        determine_outcome(&mut paper_player_1, &mut paper_player_2);
        assert_eq!(0, paper_player_1.wins);
        assert_eq!(0, paper_player_1.losses);
        assert_eq!(0, paper_player_2.wins);
        assert_eq!(0, paper_player_2.losses);
    }

    #[test]
    fn scissor_draws_scissor() {
        let weapons = weapons_map();
        let mut scissor_player_1 = Player {
            name: String::from("Scissor Player 1"),
            weapon: weapons[&String::from("scissor")],
            ..Default::default()
        };
        let mut scissor_player_2 = Player {
            name: String::from("Scissor Player 2"),
            weapon: weapons[&String::from("scissor")],
            ..Default::default()
        };
        determine_outcome(&mut scissor_player_1, &mut scissor_player_2);
        assert_eq!(0, scissor_player_1.wins);
        assert_eq!(0, scissor_player_1.losses);
        assert_eq!(0, scissor_player_2.wins);
        assert_eq!(0, scissor_player_2.losses);
    }
}
