use core::panic;
use std::cmp::Ordering;
use std::num::ParseIntError;

pub struct Guess {
    value: u32
}

pub enum GuessStatus {
    Correct,
    Greater,
    Lesser
}

impl Guess {
    pub fn from_string(input: &str) -> Result<Guess, ParseIntError> {
        let guess = input.parse::<u32>()?;

        if guess < 1 || guess > 100 {
            panic!("Guess should be in the range of 1 to 100, got {}", guess);
        }

        Ok(Guess { value: guess })
    }

    pub fn check(&self, secret_num: u32) -> GuessStatus {
        match self.value.cmp(&secret_num) {
            Ordering::Equal => GuessStatus::Correct,
            Ordering::Greater => GuessStatus::Greater,
            Ordering::Less => GuessStatus::Lesser
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}