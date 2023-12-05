use clap::{Parser, Subcommand};


pub fn figure() -> Result<String, String> {
    let cli = Cli::parse();

    let result: Result<String, String> = match cli.command {
        Some(Commands::Hello { message }) => Ok(format!("the message: {message}")),
        None => Ok("try qwit --help for information on how to use qwit".to_string()),
    };
    result
}

/// qwit cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "qwit")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// [STABLE] message
    Hello {
        #[arg(short, long, env = "Q_MESSAGE")]
        message: String,
    },

}
