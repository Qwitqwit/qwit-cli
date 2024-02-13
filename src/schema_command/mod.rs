use std::{fs::File, path::PathBuf};

use qwitlib::lines::read_file_lines;

use self::structure::Schema;

mod structure;

pub fn validation(schema_source: &str, source: PathBuf) -> Result<String, String> {
    let schema = Schema::from_file(schema_source).map_err(|errs| errs.join(""))?;
    let file = File::open(source).map_err(|err| err.to_string())?;
    let lines = read_file_lines(file).map_err(|err| format!("{err:?}"))?;
    structure::check(lines, &schema, true).map_err(|errs| errs.join("\n"))
}
