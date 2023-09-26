mod input;
mod guess;

use guess::{Guess, GuessStatus};
use rand::{Rng, thread_rng};

fn display_title() {
    println!("     _____                     _ 
    / ____|                   | |
   | |  __ _   _  ___  ___ ___| |
   | | |_ | | | |/ _ \\\\/ __/ __| |
   | |__| | |_| |  __/\\__ \\__ \\_|
    \\_____|\\__,_|\\___||___/___(_)");

    println!("\nGuessing Game: Improved\n");
    println!("The computer will choose a number between 1 and 100.\nYour objective is to correctly guess this number. Good Luck!");
    println!("--------------------\n");
}

fn show_results(guess_status: &GuessStatus) {
    match guess_status {
        GuessStatus::Correct => println!("Correct!"),
        GuessStatus::Greater => println!("Too High!"),
        GuessStatus::Lesser => println!("Too Low!")
    }

    println!("")
}

fn main() {
    display_title();

    loop {
        let mut rng = thread_rng();
        let secret_num: u32 = rng.gen_range(1..=100);

        let user_input = input::get_input(&"Your guess")
            .expect("Failed to read user input");

        let user_guess = match Guess::from_string(&user_input) {
            Ok(guess) => guess,
            Err(_) => {
                println!("");
                continue
            }
        };

        let user_result = user_guess.check(secret_num);

        show_results(&user_result);
    }
}
