use qwitlib::lines::read_lines;

const SCHEMA_SEPERATOR: &char = &';';
const ENUM_SEPERATOR: &char = &',';

pub struct Schema {
    seperator: String,
    columns: Vec<Column>,
}

pub struct Column {
    header: String,
    val_required: bool,
    col_required: bool,
    type_: Type,
}

impl Column {
    pub fn from_row(row: &str, sep: &str) -> Result<Self, String> {
        let values: Vec<String> = row
            .split(*SCHEMA_SEPERATOR)
            .map(std::string::ToString::to_string)
            .collect();

        let header = values
            .get(0)
            .ok_or("did not find anything at pos 0, header should be here".to_owned())?;
        let tipe: &String = values
            .get(1)
            .ok_or("did not find anything at pos 1, type should be here".to_owned())?;
        let col_required = values
            .get(2)
            .ok_or("did not find anything at pos 2, col_required should be here".to_owned())?;
        let val_required = values
            .get(3)
            .ok_or("did not find anything at pos 3, val_required should be here".to_owned())?;
        let misc = values
            .get(4)
            .ok_or("did not find anything at pos 4, misc should be here".to_owned())?;

        let tipe: Tipe = tipe.as_str().into();

        let header: String = if header.contains(sep) {
            return Err("header containsa a seperator, this is not allowed".to_string());
        } else {
            header.to_string()
        };
        let type_ = Type::from_tipe(tipe, misc)?;
        let col_required: bool = matches!(col_required.as_str(), "true");
        let val_required: bool = matches!(val_required.as_str(), "true");

        Ok(Self {
            header,
            val_required,
            col_required,
            type_,
        })
    }
}

// With misc included
enum Type {
    Integer(i64),
    Float(f64),
    String,
    Enum(Vec<String>),
}

impl Type {
    fn from_tipe(tipe: Tipe, misc: &str) -> Result<Self, String> {
        match tipe {
            Tipe::Integer => Ok(Type::Integer(
                misc.parse::<i64>().map_err(|err| err.to_string())?,
            )),
            Tipe::Float => Ok(Type::Float(
                misc.parse::<f64>().map_err(|err| err.to_string())?,
            )),
            Tipe::String => Ok(Type::String),
            Tipe::Enum => Ok(Type::Enum(
                misc.split(*ENUM_SEPERATOR)
                    .map(std::borrow::ToOwned::to_owned)
                    .collect(),
            )),
            Tipe::None(t) => Err(format!("Non valid type: {t}").to_string()),
        }
    }
}

enum Tipe {
    Integer,
    Float,
    String,
    Enum,
    None(String),
}

impl From<&str> for Tipe {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "integer" => Tipe::Integer,
            "float" => Tipe::Float,
            "string" => Tipe::String,
            "enum" => Tipe::Enum,
            _ => Tipe::None(value.to_owned()),
        }
    }
}

impl Schema {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let lines = read_lines(path).map_err(|err| format!("{err:?}"))?;
        Ok(lines.map_while(Result::ok).collect())
    }

    pub fn check_line(&self, row: &str, pos: usize) -> Result<(), String> {
        let mut errors: Vec<String> = vec![];

        let mut add_error = |err: &str| {
            let f = format!("on row {pos}, problem found: {err}");
            errors.push(f);
        };

        if pos == 0 {
            let splitted: Vec<&str> = row.split(&self.seperator).collect();
            for (col_pos, col) in self.columns.iter().enumerate() {
                if let Some(value) = splitted.get(col_pos) {
                    if value.to_lowercase() != col.header {
                        add_error(&format!("found column with wrong name at col {pos} should have been {} but was {value}", col.header));
                    }
                } else if col.col_required {
                    add_error(&format!(
                        "found no column at col {pos} should have been {}",
                        col.header
                    ));
                }
            }
        } else {
            let splitted: Vec<&str> = row.split(&self.seperator).collect();

            for (col_pos, col) in self.columns.iter().enumerate() {
                if let Some(value) = splitted.get(col_pos) {
                    match &col.type_ {
                        Type::Integer(_default) => {
                            if value.parse::<i64>().is_err() {
                                add_error(&format!(
                                    "value: {value} at {pos} should have been an integer"
                                ));
                            }
                        }
                        Type::Float(_default) => {
                            if value.parse::<f64>().is_err() {
                                add_error(&format!(
                                    "value: {value} at {pos} should have been an float",
                                ));
                            }
                        }
                        Type::String => (),
                        Type::Enum(values) => {
                            if !values.contains(&(*value).to_string()) {
                                add_error(&format!(
                                    "value: {value} at {pos} should have been an part of {values:?}",
                                ));
                            }
                        }
                    }
                } else if col.val_required {
                    add_error(&format!("found no value at col {pos}"));
                };
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.concat())
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
