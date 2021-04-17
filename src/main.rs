#![allow(dead_code, unused)]

use rand::Rng;
use std::io;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
struct Player<'a> {
    name: &'a str,
    choice: Rps,
}

impl<'a> Player<'a> {
    fn new(name: &'a str, choice: Rps) -> Self {
        Player { name, choice }
    }

    fn choice(&self) {
        println!(
            "Player {} `plays` {} ",
            self.name,
            match self.choice {
                Rps::Rock => "Rock".to_string(),
                Rps::Paper => "Paper".to_string(),
                Rps::Scissors => "Scissors".to_string(),
            }
        );
    }
}

#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn new(pick: &str) -> Self {
        match pick {
            "rock" => Rps::Rock,
            "paper" => Rps::Paper,
            "scissors" => Rps::Scissors,
            _ => Rps::Rock,
        }
    }
}

fn main() {
    println!("\nPlay Rock, Paper, Scissors Game");
    println!("Enter a Player name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed");
    if name.len() == 0 {
        name = "Player".to_string();
    }
    name = name.trim().to_string();

    loop {
        let computer_choice = vec![Rps::Rock, Rps::Paper, Rps::Scissors];
        let rand = rand::thread_rng().gen_range(0..=2);
        let computer = Player::new("computer", computer_choice[rand]);

        println!("\n\nEnter a choice [Rock, Paper, Scissors]: Press Ctrl+C to Exit.");
        let mut player_choice = String::new();
        io::stdin()
            .read_line(&mut player_choice)
            .expect("failed player choice!");
        player_choice = player_choice.trim().to_lowercase().to_string();

        // to see if player choice exist
        // if not `Rock` is chosen for the player
        if computer_choice.contains(&Rps::new(&player_choice)) {
            let player = Player::new(&name, Rps::new(&player_choice));
            play(player, computer);
        } else {
            println!("Your choice or option is invalid");
        }
    }
}

fn play(first_player: Player, second_player: Player) {
    match (first_player.choice, second_player.choice) {
        (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Rock) => {
            let (winner, loser) = if first_player.choice == Rps::Rock {
                (second_player.name, first_player.name)
            } else {
                (first_player.name, second_player.name)
            };
            first_player.choice();
            second_player.choice();
            println!("Paper Covers Rock!");
            println!("Player {:?} WINS.\nPlayer {:?} LOOSES!!", winner, loser)
        }
        (Rps::Rock, Rps::Scissors) | (Rps::Scissors, Rps::Rock) => {
            let (winner, losser) = if first_player.choice == Rps::Rock {
                (first_player.name, second_player.name)
            } else {
                (second_player.name, first_player.name)
            };
            first_player.choice();
            second_player.choice();
            println!("Rock breaks Scissors");
            println!("Player {:?} WINS.\nPlayer {:?} LOOSES!!", winner, losser)
        }
        (Rps::Paper, Rps::Scissors) | (Rps::Scissors, Rps::Paper) => {
            let (winner, losser) = if first_player.choice == Rps::Scissors {
                (first_player.name, second_player.name)
            } else {
                (second_player.name, first_player.name)
            };
            first_player.choice();
            second_player.choice();
            println!("Scissors cuts Paper");
            println!("Player {:?} WINS.\nPlayer {:?} LOOSES!!", winner, losser)
        }
        (_, _) => {
            first_player.choice();
            second_player.choice();
            println!("A TIE");
        }
    };
}
