use std::{
    fs::File,
    io::{Error, Read, Write, stdin, stdout},
};

use dialoguer::Input;

pub fn question(question: &str) -> String {
    let result: String = Input::new()
        .with_prompt(format!("{}", question))
        .interact_text()
        .unwrap();
    result.trim().to_string()
}

pub fn parse_buffer(path: &str) -> Result<Vec<String>, Error> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.lines().map(|l| l.to_string()).collect())
}

pub fn back() {
    let mut input = String::new();

    print!("\nPress Enter to return to menu...");
    stdout().flush().unwrap();

    stdin().read_line(&mut input).unwrap();
}
