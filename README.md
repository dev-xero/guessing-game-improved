# Guessing Game: Improved
```rust
   _____                     _ 
  / ____|                   | |
 | |  __ _   _  ___  ___ ___| |
 | | |_ | | | |/ _ \/ __/ __| |
 | |__| | |_| |  __/\__ \__ \_|
  \_____|\__,_|\___||___/___(_)
```
An improved implementation of the original guessing game, still written in Rust.  
You have to guess a number between 1 and 100 and the program tells you whether you got it right or not and repeats this infinitely (or more likely till you run out of RAM).

## Improvements
1. Separated input and guessing logic to different modules.
2. Specific functions return a `Result<T, E>` which allows more flexibility to the calling code.
3. Exit the program if the guess is not in the specified range (could instead just repeat).
4. Added cool ASCII art.

## Implementations
  ### 1. Guessing Functionality
  Improvements were made to the guessing functionality of the program.
    
   ```rust
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
    }
   ```
### 2. Input Handling
A few tweaks to the original input handling code, including flushing the output stream before receiving input.

```rust
pub fn get_input(msg: &str) -> Result<String, io::Error> {
    let mut input_buffer = String::new();

    print!("{}: ", msg);
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut input_buffer)?;

    Ok(input_buffer.trim().to_string())
}
```
