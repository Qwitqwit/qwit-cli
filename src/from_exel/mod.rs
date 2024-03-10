use calamine::{open_workbook_auto, Data, Range, Reader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn all(source: &PathBuf, target: &PathBuf) -> Result<String, String> {
    read(source, target)
}

fn read(source: &PathBuf, target: &PathBuf) -> Result<String, String> {
    // find source file
    let sce = PathBuf::from(source);
    match sce.extension().and_then(|s| s.to_str()) {
        Some("xlsx" | "xlsm" | "xlsb" | "xls") => (),
        _ => panic!("Expecting an excel file"),
    }

    // create or append to target file
    let target_path = PathBuf::from(target).with_extension("csv");
    let target_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(target_path)
        .unwrap();
    let mut target = BufWriter::new(target_file);

    // open xl file
    let mut xl = open_workbook_auto(&sce).unwrap();
    let range = xl.worksheet_range_at(0).unwrap().unwrap();

    write_range(&mut target, &range, PrintOperator {})
        .map(|()| "All done".to_owned())
        .map_err(|err| err.0)
}

fn write_range<W: Write>(
    _target: &mut W,
    range: &Range<Data>,
    operator: impl CsvRowOperator,
) -> Result<(), CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    operator.operate(all_rows)
}

#[derive(Debug)]
struct CsvError(String);

trait CsvRowOperator {
    fn operate(
        self,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError>;
}

struct PrintOperator;

impl CsvRowOperator for PrintOperator {
    fn operate(
        self,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError> {
        rows.for_each(|r| {
            println!("---new row---");
            r.for_each(|v| match v.0 {
                Ok(k) => println!("{k}"),
                Err(e) => println!("{e}"),
            });
        });
        Ok(())
    }
}

struct CsvRow;

impl CsvRow {
    fn iterator(value: &[Data]) -> impl Iterator<Item = CsvValue> + '_ {
        value.iter().cloned().map(|c| {
            let v: CsvValue = c.into();
            v
        })
        // done, new line
        // write!(target, "\r\n")?;
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
