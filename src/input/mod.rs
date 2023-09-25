use std::io::{self, Write};

pub fn get_input(msg: &str) -> Result<String, io::Error> {
    let mut input_buffer = String::new();

    print!("{:?}: ", msg);
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut input_buffer)?;

    Ok(input_buffer.trim().to_string())
}