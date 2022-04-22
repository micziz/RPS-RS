// Rock paper scissors

use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock Paper Scissors!");
    println!("The rules are simple! \nChoose your weapon and see who wins!\nRock beats scissors, scissors beats paper, and paper beats rock!");
    println!("Use 0 for rock, 1 for paper, and 2 for scissors.");
    println!("Enter your choice: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u32 = user_input.trim().parse().expect("Please enter a number!");
    let mut rng = rand::thread_rng();
    let computer_choice: u32 = rng.gen_range(0..2);
    println!("Computer chose {}", computer_choice);
    if user_input == computer_choice {
        println!("It's a tie!");
    } else if user_input == 0 && computer_choice == 1 || user_input == 1 && computer_choice == 2 || user_input == 2 && computer_choice == 0 {
        println!("You lose!");
    } else {
        println!("GG! You win!");
    }

}
