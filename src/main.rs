#![warn(clippy::pedantic)]


mod commands;

fn main() {
    match commands::figure() {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("{error_message}");
            std::process::exit(1)
        }
    }
}
