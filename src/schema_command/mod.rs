use std::fs::File;

use qwitlib::lines::read_file_lines;

use self::structure::Schema;

mod structure;

pub fn validation(schema_source: &str, source: String, num: Option<i64>) -> Result<String, String> {
    let schema = Schema::from_file(schema_source)?;
    let file = File::open(source).map_err(|err| err.to_string())?;
    let lines = read_file_lines(file).map_err(|err| format!("{err:?}"))?;
    structure::check(lines, &schema, num)
}
