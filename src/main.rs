mod input;
mod guess;

use guess::Guess;

fn main() {
    println!("Guessing Game: Improved");

    let user_input = input::get_input(&"Your guess")
        .expect("Failed to read user guess");

    let user_guess = Guess::from_string(&user_input)
        .expect("Guess should be a positive number");

    println!("{}", user_guess.value());
}
