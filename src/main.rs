use std::io;

mod player;
mod round;
mod weapons;

pub use player::Player;
pub use weapons::weapons_map;
pub use weapons::Weapon;
pub use weapons::WeaponKind;

fn main() {
    println!("Enter your player's name:");
    let mut p1_name = String::new();
    io::stdin()
        .read_line(&mut p1_name)
        .expect("failed to read name.");

    let players: Vec<Player> = player::render_players(p1_name.trim().to_string());
    let mut player1 = players[0].clone();
    let mut player2 = players[1].clone();
    let mut game_iteration: i32 = 0;

    loop {
        if player1.wins == 3 || player2.wins == 3 {
            let winner: Player = if player1.wins > player2.wins {
                player1
            } else {
                player2
            };
            println!("Game over! {} wins!", winner.name);
            break;
        };

        game_iteration += 1;

        round::handler(&mut player1, &mut player2, game_iteration);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "\n{}: Wins = {} Losses = {}\nRobot: Wins = {} Losses = {}",
            String::from(p1_name.trim()),
            player1.wins,
            player1.losses,
            player2.wins,
            player2.losses,
        );
    }
}
