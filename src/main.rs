extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("What number am I thinking of?");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Psst, it's: {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
