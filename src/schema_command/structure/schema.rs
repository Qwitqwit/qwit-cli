use qwitlib::lines::read_lines;

use super::{col::Column, type_::Type};

pub struct Schema {
    seperator: String,
    columns: Vec<Column>,
}

impl Schema {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let lines = read_lines(path).map_err(|err| format!("{err:?}"))?;
        Ok(lines.map_while(Result::ok).collect())
    }

    pub fn check_line(&self, row: &str, line_pos: usize) -> Result<(), String> {
        let mut errors: Vec<String> = vec![];

        let add_error = |err: &str| {
            let f = format!("on row {line_pos}, problem found: {err}");
            errors.push(f);
        };

        if line_pos == 0 {
            self.check_header(row, line_pos, add_error);
        } else {
            self.check_normal_line(row, line_pos, add_error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.concat())
        }
    }

    fn check_header(&self, row: &str, line_pos: usize, mut add_error: impl FnMut(&str)) {
        let splitted: Vec<&str> = row.split(&self.seperator).collect();
        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.get(col_pos) {
                if value.to_lowercase() != col.header {
                    add_error(&format!("found column with wrong name at col {line_pos} should have been {} but was {value}", col.header));
                }
            } else if col.col_required {
                add_error(&format!(
                    "found no column at col {col_pos} should have been {}",
                    col.header
                ));
            }
        }
    }

    fn check_normal_line(&self, row: &str, line_pos: usize, mut add_error: impl FnMut(&str)) {
        let splitted: Vec<&str> = row.split(&self.seperator).collect();

        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.get(col_pos) {
                Self::check_types(col, value, line_pos, &mut add_error);
            } else if col.val_required {
                add_error(&format!("found no value at col {col_pos}"));
            };
        }
    }

    fn check_types(col: &Column, value: &&str, line_pos: usize, mut add_error: impl FnMut(&str)) {
        match &col.type_ {
            Type::Integer(_default) => {
                if value.parse::<i64>().is_err() {
                    add_error(&format!(
                        "value: {value} at {line_pos} should have been an integer"
                    ));
                }
            }
            Type::Float(_default) => {
                if value.parse::<f64>().is_err() {
                    add_error(&format!(
                        "value: {value} at {line_pos} should have been an float",
                    ));
                }
            }
            Type::String => (),
            Type::Enum(values) => {
                if !values.contains(&(*value).to_string()) {
                    add_error(&format!(
                        "value: {value} at {line_pos} should have been an part of {values:?}",
                    ));
                }
            }
        }
    }
}

impl FromIterator<String> for Schema {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut seperator = String::new();
        let mut columns: Vec<Column> = vec![];

        for (pos, col_row) in iter.into_iter().enumerate() {
            if pos == 0 {
                let first_line: Vec<String> = col_row
                    .split("sep=")
                    .map(std::borrow::ToOwned::to_owned)
                    .collect();
                seperator = first_line[0].clone();
            } else {
                match Column::from_row(&col_row, &seperator) {
                    Ok(s_col) => columns.push(s_col),
                    Err(err) => println!("found wrong column description at row {pos} for {err}"),
                }
            }
        }

        Self { seperator, columns }
    }
}
