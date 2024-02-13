#![warn(clippy::pedantic)]

mod commands;
mod schema_command;
mod show;

fn main() {
    match commands::figure() {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("qwit-error: {error_message}");
            std::process::exit(1)
        }
    }
}
