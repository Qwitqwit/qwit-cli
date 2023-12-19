use qwitlib::lines::read_lines;

use crate::schema_command;

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

    pub fn check_line(
        &self,
        row: &str,
        line_pos: usize,
    ) -> Result<(), Box<schema_command::structure::SchemaError>> {
        let splitted: Vec<String> = row
            .split(&self.seperator)
            .map(std::string::ToString::to_string)
            .collect();

        if line_pos == 0 {
            self.check_header(&splitted, line_pos)?;
        } else {
            self.check_normal_line(&splitted, line_pos)?;
        }
        Ok(())
    }

    fn check_header(
        &self,
        splitted: &[String],
        line_pos: usize,
    ) -> Result<(), Box<schema_command::structure::SchemaError>> {
        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.get(col_pos) {
                if value.to_lowercase() != col.header.to_lowercase() {
                    return Err(Box::new(SchemaError::Header {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong header".to_owned(),
                        row_: splitted.to_owned(),
                        sep: self.seperator.clone(),
                    }));
                }
            } else if col.col_required {
                return Err(Box::new(SchemaError::Column {
                    row_pos: line_pos,
                    col_pos,
                    header: col.header.clone(),
                    descripton: "column missing".to_owned(),
                    row_: splitted.to_owned(),
                    sep: self.seperator.clone(),
                }));
            }
        }

        Ok(())
    }

    fn check_normal_line(
        &self,
        splitted: &[String],
        line_pos: usize,
    ) -> Result<(), Box<schema_command::structure::SchemaError>> {
        for (col_pos, col) in self.columns.iter().enumerate() {
            if let Some(value) = splitted.get(col_pos) {
                self.check_types(col, value, line_pos, col_pos, splitted)?;
            } else if col.val_required {
                return Err(Box::new(SchemaError::ValueMissing {
                    row_pos: line_pos,
                    col_pos,
                    header: col.header.clone(),
                    descripton: "value missing".to_owned(),
                    row_: splitted.to_owned(),
                    sep: self.seperator.clone(),
                }));
            };
        }
        Ok(())
    }

    fn check_types(
        &self,
        col: &Column,
        value: &str,
        line_pos: usize,
        col_pos: usize,
        splitted: &[String],
    ) -> Result<(), Box<schema_command::structure::SchemaError>> {
        match &col.type_ {
            Type::Integer(default) => {
                if value.parse::<i64>().is_err() && !value.is_empty() {
                    return Err(Box::new(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted.to_owned(),
                        sep: self.seperator.clone(),
                        type_: Type::Integer(*default),
                    }));
                }
            }
            Type::Float(default) => {
                if value.parse::<f64>().is_err() && !value.is_empty() {
                    return Err(Box::new(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted.to_owned(),
                        sep: self.seperator.clone(),
                        type_: Type::Float(*default),
                    }));
                }
            }
            Type::String => (),
            Type::Enum(values) => {
                if !values.contains(&(*value).to_string()) {
                    return Err(Box::new(SchemaError::Type {
                        row_pos: line_pos,
                        col_pos,
                        header: col.header.clone(),
                        descripton: "wrong type".to_owned(),
                        row_: splitted.to_owned(),
                        sep: self.seperator.clone(),
                        type_: Type::Enum(values.clone()),
                    }));
                }
            }
        }
        Ok(())
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
