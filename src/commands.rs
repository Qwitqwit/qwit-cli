use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::schema_command::validation;
use crate::show;

pub fn figure() -> Result<String, String> {
    let cli = Cli::parse();

    let result: Result<String, String> = match cli.command {
        Some(Commands::Validation { schema, source }) => validation(
            &schema,
            std::string::String::combine_with_working_dir(source, cli.working_dir),
        ),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::Show { source, num }) => show::csv(
            &std::string::String::combine_with_working_dir(source, cli.working_dir),
            num,
        ),
        None => Ok("try qwit --help for information on how to use qwit".to_string()),
    };

    result
}

trait Combiner {
    fn combine_with_working_dir(path: Self, working_dir: Option<String>) -> PathBuf;
}

impl Combiner for String {
    fn combine_with_working_dir(path: Self, working_dir: Option<String>) -> PathBuf {
        let actual = PathBuf::from(path);
        let Some(working_dir) = working_dir else {
            return actual;
        };
        let dir_path = PathBuf::from(working_dir);
        dir_path.join(actual)
    }
}

/// qwit cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "qwit")]
struct Cli {
    #[arg(short, long, env = "Q_SEP", default_value_t = String::from(";"))]
    sep: String,

    #[arg(short, long, env = "Q_WORKING_DIR", default_value = Option::None)]
    working_dir: Option<String>,

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
    /// [STABLE] validate a dsv file against a dsv schema
    Validation {
        #[arg(short, long, env = "Q_SCHEMA_SOURCE")]
        schema: String,
        #[arg(short, long, env = "Q_SOURCE")]
        source: String,
    },
}
