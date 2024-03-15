use calamine::Data;
use std::path::PathBuf;

use self::read_n_x::read;

mod operators;
mod read_n_x;

pub fn all(source: &PathBuf, target: &PathBuf, sep: &str) -> Result<String, String> {
    read(source, target, sep)
}

#[derive(Debug)]
struct CsvError(String);

trait CsvRowOperator {
    fn operate(
        &mut self,
        separator: String,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError>;
}

struct CsvRow;

impl CsvRow {
    fn iterator(value: &[Data]) -> impl Iterator<Item = CsvValue> + '_ {
        value.iter().cloned().map(|c| {
            let v: CsvValue = c.into();
            v
        })
    }
}

#[derive(Debug, Clone)]
struct CsvValue(Result<String, String>);

impl From<Data> for CsvValue {
    fn from(value: Data) -> Self {
        match value {
            // we do nothing on empty
            Data::Empty => CsvValue(Err("Empty Value".to_owned())),
            // we write for those types
            Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                // we replace ; with nothing
                let escaped = s.replace(';', "");
                CsvValue(Ok(escaped))
            }
            Data::DateTime(ref f) => CsvValue(Ok(f.to_string())),

            Data::Float(ref f) => CsvValue(Ok(f.to_string())),
            // we also just write for those
            Data::Int(ref i) => CsvValue(Ok(i.to_string())),
            Data::Bool(ref b) => CsvValue(Ok(b.to_string())),
            Data::Error(ref e) => CsvValue(Err(format!(
                "error in sheet, fix or remove cell error: {e}"
            ))),
        }
    }
}
