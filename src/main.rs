extern crate rand;

use std::io;
use rand::Rng;


fn main() {
    
    loop {
    println!("Rock-Paper-Scissors, you choose");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("failed");
    let guess = guess.trim();
    

    let computer_choice = rand::thread_rng().gen_range(1, 4);
if computer_choice == 1 && guess == "rock" {
    println!("computer chooses rock its a tie!");
} else if computer_choice == 1 && guess == "paper" {
    println!("computer chooses rock you WIN!!!");
} else if computer_choice == 1 && guess == "scissors" {
    println!("computer chooses rock You LOOSE!!!");
} else if computer_choice == 2 && guess == "rock" {
    println!("computer chooses paper you LOOSE!!!");
} else if computer_choice == 2 && guess == "paper" {
    println!("computer chooses paper a tie");
} else if computer_choice == 2 && guess == "scissors" {
    println!("computer chooses paper you WIN!!!");
} else if computer_choice == 3 && guess == "rock" {
    println!("computer chooses scissors you WIN!!!");
} else if computer_choice == 3 && guess == "paper" {
    println!("computer chooses scissors you LOOSE!!!");
} else if computer_choice == 3 && guess == "scissors" {
    println!("computer chooses scissors a tie!")
}
}
}