use std::{
    fs::File,
    io::{BufReader, Lines},
    num::TryFromIntError,
};

mod col;
mod schema;
mod type_;

pub use schema::Schema;

const RES: &str = r"all good

_________check done_________";

pub fn check(
    lines: Lines<BufReader<File>>,
    schema: &schema::Schema,
    num: Option<i64>,
) -> Result<String, String> {
    let sized_lines = match num {
        Some(num) => lines.take(
            num.try_into()
                .map_err(|err: TryFromIntError| err.to_string())?,
        ),
        None => lines.take(usize::MAX),
    };

    let mut errors: Vec<String> = vec![];

    sized_lines.enumerate().for_each(|(n, line)| {
        if let Ok(line) = line {
            let error = schema.check_line(&line, n);

            if let Err(err) = error {
                errors.push(err);
            }
        }
    });

    if errors.is_empty() {
        Ok(RES.to_owned())
    } else {
        Err(format!("Errors: {}", errors.join("/n")))
    }
}
