use std::io;
use std::fmt;
use rand::seq::SliceRandom;


fn main() {
    println!("Rock Paper Scissors!");

    // Define players
    let mut player1 = Player {
        name: "Antonio".to_string(),
        wins: 0,
        losses: 0,
    };
    let mut player2 = Player {
        name: "Robot".to_string(),
        wins: 0,
        losses: 0,
    };

    loop {
        game(&mut player1, &mut player2);
        println!("\n{}\n", player1);
        println!("{}\n", player2);
    }
}

fn game(mut p1: &mut Player, mut p2: &mut Player) {
    println!("Choose your weapon: \n1. Rock\n2. Paper\n3. Scissor\n\n");
    let mut p1_choice = String::new();
    io::stdin()
        .read_line(&mut p1_choice)
        .expect("Failed to read choice.");
    let p1_choice = p1_choice.trim().to_string();
    
    let weapons = vec!["Rock", "Paper", "Scissor"];
    let p2_choice = weapons.choose(&mut rand::thread_rng()).unwrap().to_string();

    let winner = who_wins(p1_choice, p2_choice);
    if winner == "Player1" {
        p1.wins = p1.wins + 1;
        p2.losses = p2.losses + 1;
    } else if winner == "Player2" {
        p2.wins = p2.wins + 1;
        p1.losses = p1.losses + 1;
    } else {
        //pass
    }
}

fn who_wins(choice1: String, choice2: String) -> String {
    println!("{}", choice2);
    if choice1 == String::from("Rock") {
        if choice2 == String::from("Rock") {
            return "Draw".to_string();
        } else if choice2 == "Paper".to_string() {
           return "Player2".to_string();
        } else if choice2 == "Scissor".to_string() {
            return "Player1".to_string();
        } else {
            return "Invalid Game".to_string();
        }
    } else if choice1 == "Paper".to_string() {
        if choice2 == "Paper".to_string() {
            return "Draw".to_string();
        } else if choice2 == "Rock".to_string() {
            return "Player1".to_string();
        } else if choice2 == "Scissor".to_string() {
            return "Player2".to_string();
        } else {
            return "Invalid Game".to_string();
        } 
    }

    else if choice1 == "Scissor".to_string() {
        if choice2 == "Scissor".to_string() {
            return "Draw".to_string();
        } else if choice2 == "Paper".to_string() {
            return "Player1".to_string();
        } else if choice2 == "Rock".to_string() {
            return "Player2".to_string();
        } else {
            return "Invalid Game".to_string();
        } 
    }

    else {
        return "Invalid Game".to_string();
    }
}

struct Player {
    name: String,
    wins: u32,
    losses: u32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Player Name: {}, Wins: {}, Losses: {})", self.name, self.wins, self.losses)
    }
}
