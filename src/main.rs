mod input;
mod guess;

use guess::Guess;

fn main() {
    println!("Guessing Game: Improved");

    loop {
        let user_input = input::get_input(&"Your guess")
            .expect("Failed to read user input");

        let user_guess = match Guess::from_string(&user_input) {
            Ok(guess) => guess,
            Err(_) => {
                println!("");
                continue
            }
        };

        println!("{}", user_guess.value());
        println!("")
    }
}
