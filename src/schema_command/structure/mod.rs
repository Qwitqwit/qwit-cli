use std::{
    fs::File,
    io::{BufReader, Lines},
};

mod col;
mod schema;
mod type_;

pub use schema::Schema;

use self::type_::Type;

const RES: &str = r"
_________check done_________";

pub fn check(
    lines: Lines<BufReader<File>>,
    schema: &schema::Schema,
    print: bool,
) -> Result<String, Vec<String>> {
    let result: Vec<_> = lines
        .filter_map(|res| match res {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .filter(|l| !l.starts_with("sep="))
        .enumerate()
        .filter(|(_, l)| !l.starts_with("sep="))
        .map(|(n, line)| {
            let res = schema.check_line(&line, n);

            if print && n % 100_000 == 0 {
                if let Err(err) = res.clone() {
                    println!("--at line {n} found err {err:#?}");
                } else {
                    println!("--at line {n} no errors");
                }
            }
            res
        })
        .filter_map(std::result::Result::err)
        .map(|err| err.message())
        .collect();

    if result.is_empty() {
        Ok(RES.to_owned())
    } else {
        Err(result.into_iter().collect())
    }
}

#[derive(Debug, Clone)]
pub enum SchemaError {
    Header {
        row_pos: usize,
        col_pos: usize,
        header: String,
        descripton: String,
        row_: Vec<String>,
        sep: String,
    },
    Column {
        row_pos: usize,
        col_pos: usize,
        header: String,
        descripton: String,
        row_: Vec<String>,
        sep: String,
    },
    ValueMissing {
        row_pos: usize,
        col_pos: usize,
        header: String,
        descripton: String,
        row_: Vec<String>,
        sep: String,
    },
    Type {
        row_pos: usize,
        col_pos: usize,
        header: String,
        type_: Type,
        descripton: String,
        row_: Vec<String>,
        sep: String,
    },
}
impl SchemaError {
    pub fn message(&self) -> String {
        match self {
            SchemaError::Header {
                row_pos,
                col_pos,
                header,
                descripton,
                row_,
                sep
            } => format!("HeaderError-> header: {header}, row: {row_pos}, col: {col_pos}, description: {descripton}  row: {}", row_.to_pos_error(*col_pos, sep)),
            SchemaError::Column {
                row_pos,
                col_pos,
                header,
                descripton,
                row_,
                sep
            } => format!("ColumnError-> header: {header}, row: {row_pos}, col: {col_pos}, description: {descripton}  row: {}", row_.to_pos_error(*col_pos, sep)),
            SchemaError::ValueMissing {
                row_pos,
                col_pos,
                header,
                descripton,
                row_,
                sep
            } => format!("ValueMissingError-> header: {header}, row: {row_pos}, col: {col_pos}, description: {descripton} row: {}", row_.to_pos_error(*col_pos, sep)),
            SchemaError::Type {
                row_pos,
                col_pos,
                header,
                type_,
                descripton,
                row_,
                sep
            } => format!("TypeError-> header: {header}, expected type: {type_:?} row: {row_pos}, col: {col_pos}, description: {descripton} row: {}", row_.to_pos_error(*col_pos, sep))
            }
    }
}

trait PositionedErrorString {
    fn to_pos_error(&self, col_pos: usize, sep: &str) -> String;
}

impl PositionedErrorString for Vec<String> {
    fn to_pos_error(&self, col_pos: usize, sep: &str) -> String {
        let splitted = self.split_at(col_pos);
        let mut first = splitted.0.to_vec();
        let second = splitted.1;
        let splitted_second: (&[String], &[String]) = second.split_at(1);
        let error_value = splitted_second.0.first();
        let mut third = splitted_second.1.to_vec();

        let value = match error_value {
            Some(value) => format!("->{}<-", &value),
            None => String::new(),
        };

        first.push(value);
        first.append(&mut third);
        first.join(sep)
    }
}
