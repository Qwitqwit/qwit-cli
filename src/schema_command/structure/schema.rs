use qwitlib::lines::read_lines;

use super::{col::Column, type_::Type, SchemaError};

pub struct Schema {
    seperator: String,
    columns: Vec<Column>,
}

impl Schema {
    pub fn from_file(path: &str) -> Result<Self, Vec<String>> {
        let lines = read_lines(path).map_err(|err| vec![format!("{err:?}")])?;
        Ok(lines.map_while(Result::ok).collect())
    }

    pub fn check_line(&self, row: &str, line_pos: usize) -> Result<(), Vec<SchemaError>> {
        let mut errors: Vec<SchemaError> = vec![];

        if line_pos == 0 {
            let res = self.check_header(row, line_pos);
            if let Err(mut err) = res {
                errors.append(&mut err);
            }
        } else {
            let res = self.check_normal_line(row, line_pos);
            if let Err(mut err) = res {
                errors.append(&mut err);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_header(&self, row: &str, line_pos: usize) -> Result<(), Vec<SchemaError>> {
        let mut errors: Vec<SchemaError> = vec![];

        let splitted: Vec<String> = row
            .split(&self.seperator)
            .map(std::string::ToString::to_string)
            .collect();

        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.clone().get(col_pos) {
                if value.to_lowercase() != col.header.to_lowercase() {
                    errors.push(SchemaError::Header {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong header".to_owned(),
                        row_: splitted.clone(),
                        sep: self.seperator.clone(),
                    });
                }
            } else if col.col_required {
                errors.push(SchemaError::Column {
                    row_pos: line_pos,
                    col_pos,
                    header: col.header.clone(),
                    descripton: "column missing".to_owned(),
                    row_: splitted.clone(),
                    sep: self.seperator.clone(),
                });
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_normal_line(&self, row: &str, line_pos: usize) -> Result<(), Vec<SchemaError>> {
        let mut errors: Vec<SchemaError> = vec![];

        let splitted: Vec<String> = row
            .split(&self.seperator)
            .map(std::string::ToString::to_string)
            .collect();
        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.clone().get(col_pos) {
                let res = self.check_types(col, value, line_pos, col_pos, splitted.clone());
                if let Err(mut err) = res {
                    errors.append(&mut err);
                }
            } else if col.val_required {
                errors.push(SchemaError::ValueMissing {
                    row_pos: line_pos,
                    col_pos,
                    header: col.header.clone(),
                    descripton: "value missing".to_owned(),
                    row_: splitted.clone(),
                    sep: self.seperator.clone(),
                });
            };
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_types(
        &self,
        col: &Column,
        value: &str,
        line_pos: usize,
        col_pos: usize,
        splitted: Vec<String>,
    ) -> Result<(), Vec<SchemaError>> {
        let mut errors: Vec<SchemaError> = vec![];

        match &col.type_ {
            Type::Integer(default) => {
                if value.parse::<i64>().is_err() && !value.is_empty() {
                    errors.push(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted,
                        sep: self.seperator.clone(),
                        type_: Type::Integer(*default),
                    });
                }
            }
            Type::Float(default) => {
                if value.parse::<f64>().is_err() && !value.is_empty() {
                    errors.push(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted,
                        sep: self.seperator.clone(),
                        type_: Type::Float(*default),
                    });
                }
            }
            Type::String => (),
            Type::Enum(values) => {
                if !values.contains(&(*value).to_string()) {
                    errors.push(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted,
                        sep: self.seperator.clone(),
                        type_: Type::Enum(values.clone()),
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
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

                seperator = first_line[1].clone();
                println!("seperator found: {seperator}");
            } else if pos > 1 {
                match Column::from_row(&col_row, &seperator) {
                    Ok(s_col) => columns.push(s_col),
                    Err(err) => {
                        if !col_row.is_empty() {
                            println!("found wrong column description at row {pos} for {err}, line {col_row}");
                        }
                    }
                }
            }
        }

        println!("schema done {columns:#?}");
        Self { seperator, columns }
    }
}
