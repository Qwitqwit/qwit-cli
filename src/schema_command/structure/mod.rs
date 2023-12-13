use std::{
    fs::File,
    io::{BufReader, Lines},
    num::TryFromIntError,
};

mod col;
mod schema;
mod type_;

use rand::seq::SliceRandom;
pub use schema::Schema;

const RES: &str = r"
_________check done_________";

pub fn check(
    lines: Lines<BufReader<File>>,
    schema: &schema::Schema,
    num: Option<i64>,
    print: bool,
) -> Result<String, Vec<String>> {
    let mut rng = rand::thread_rng();

    let sized_lines = match num {
        Some(num) => lines.take(
            num.try_into()
                .map_err(|err: TryFromIntError| err.to_string())
                .map_err(|err| vec![err])?,
        ),
        None => lines.take(usize::MAX),
    };

    let mut errors: Vec<String> = vec![];

    sized_lines
        .filter_map(std::result::Result::ok)
        .filter(|l| !l.starts_with("sep="))
        .enumerate()
        .for_each(|(n, line)| {
            let error = schema.check_line(&line, n);

            if let Err(mut err) = error {
                errors.append(&mut err);
            }

            if print && n % 100_000 == 0 {
                let err = errors.choose(&mut rng);
                println!(
                    "--at line {n}, {} errors in total, one of them {:#?}",
                    errors.len(),
                    err.unwrap_or(&"None".to_owned())
                );
            }
        });

    if errors.is_empty() {
        Ok(RES.to_owned())
    } else {
        Err(errors)
    }
}
