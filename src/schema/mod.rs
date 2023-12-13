use std::{fs::File, num::TryFromIntError};

use qwitlib::lines::read_file_lines;

use self::structure::Schema;

mod structure;

pub fn validation(schema_source: &str, source: String, num: Option<i64>) -> Result<String, String> {
    let schema = Schema::from_file(schema_source)?;

    check(source, &schema, num)
}

fn check(source: String, schema: &Schema, num: Option<i64>) -> Result<String, String> {
    let file = File::open(source).map_err(|err| err.to_string())?;
    let lines = read_file_lines(file).map_err(|err| format!("{err:?}"))?;

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
        println!("file complies to schema");
    } else {
        println!("Errors: {}", errors.join("/n"));
    }
    println!();
    Ok("_________check done_________".to_owned())
}
