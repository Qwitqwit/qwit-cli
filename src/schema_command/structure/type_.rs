const ENUM_SEPERATOR: &char = &',';

// With misc included
#[derive(Debug, Clone)]
pub enum Type {
    Integer(i64),
    Float(f64),
    String,
    Enum(Vec<String>),
}

impl Type {
    pub fn from_tipe(tipe: Tipe, misc: &str) -> Result<Self, String> {
        match tipe {
            Tipe::Integer => Ok(Type::Integer(if misc.is_empty() {
                -1
            } else {
                misc.parse::<i64>().map_err(|err| err.to_string())?
            })),
            Tipe::Float => Ok(Type::Float(if misc.is_empty() {
                -1.0
            } else {
                misc.parse::<f64>().map_err(|err| err.to_string())?
            })),
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

pub enum Tipe {
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
