use std::{
    fs::File,
    io::{BufWriter, Write},
};

use super::{CsvError, CsvRowOperator, CsvValue};

pub struct PrintOperator;

impl CsvRowOperator for PrintOperator {
    fn operate(
        &mut self,
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

pub struct FileWritingOperator {
    pub(crate) writer: BufWriter<File>,
}

impl CsvRowOperator for FileWritingOperator {
    fn operate(
        &mut self,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError> {
        let _ = rows.map(|r| {
            let values: Vec<String> = r.filter_map(|v| v.0.ok()).collect();
            let len = values.len();

            let _ = values.iter().enumerate().map(|(n, v)| {
                self.write(v);
                if n != len {
                    self.sep(";");
                }
            });

            self.end_line();
        });
        Ok(())
    }
}

impl FileWritingOperator {
    fn write(&mut self, value: &str) {
        let _ = write!(&mut self.writer, "{value}").map_err(|err| CsvError(err.to_string()));
    }
    fn end_line(&mut self) {
        write!(self.writer, "\r\n").unwrap();
    }
    fn sep(&mut self, sep: &str) {
        write!(self.writer, "{sep}").unwrap();
    }
}
