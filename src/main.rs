use rand::Rng;
use std::io;

fn main() {
    let rndNumber = rand::thread_rng().gen_range(1, 100);

    println!("Guess the number!");
    println!("Input your guess:");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
