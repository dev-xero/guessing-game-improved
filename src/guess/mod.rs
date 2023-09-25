use core::panic;
use std::num::ParseIntError;

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn from_string(input: &str) -> Result<Guess, ParseIntError> {
        let guess = input.parse::<u32>()?;

        if guess < 1 || guess > 100 {
            panic!("Guess should be in the range of 1 to 100, got {}", guess);
        }

        Ok(Guess { value: guess })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}