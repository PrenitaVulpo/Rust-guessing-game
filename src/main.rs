use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let rnd_number = rand::thread_rng().gen_range(1, 101);

    println!("debug: {}", rnd_number);

    game_loop(rnd_number);
}

fn game_loop(rnd_number: u32) {
    loop {
        println!("Input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please guess a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&rnd_number) {
            Ordering::Less => println!("{}", "That's smaller!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "That's bigger!".red()),
        }
    }
}
