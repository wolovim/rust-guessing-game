use std::io;

fn main() {
    println!("What number am I thinking of?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
