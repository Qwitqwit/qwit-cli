use clap::{Parser, Subcommand};

use crate::show;

pub fn figure() -> Result<String, String> {
    let cli = Cli::parse();

    let result: Result<String, String> = match cli.command {
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::Show { source, num }) => show::csv(source, num),
        None => Ok("try qwit --help for information on how to use qwit".to_string()),
    };

    result
}

/// qwit cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "qwit")]
struct Cli {
    #[arg(short, long, env = "Q_SEP", default_value_t = String::from(";"))]
    sep: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// [STABLE] print markdown doc of qwit to std out
    Markdown,

    /// [STABLE] show the dsv from the start in a nice way
    Show {
        #[arg(short, long, env = "Q_SOURCE")]
        source: String,
        #[arg(short, long, env = "Q_NUM", default_value_t = 100)]
        num: i64,
    },
}
