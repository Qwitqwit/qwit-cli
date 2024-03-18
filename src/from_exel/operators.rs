use std::{
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use super::{CsvError, CsvRowOperator, CsvValue};

pub struct PrintOperator;

impl CsvRowOperator for PrintOperator {
    fn operate(
        &mut self,
        _separator: String,
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
        separator: String,
        rows: impl Iterator<Item = impl Iterator<Item = CsvValue>>,
    ) -> Result<(), CsvError> {
        let size_hint = rows.size_hint().0;
        let pb = ProgressBar::new(size_hint.try_into().expect("size hint is too big"));

        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.dim.bold} [{elapsed_precise}] [{wide_bar:.}] ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap();
            }),
        );

        rows.for_each(|r| {
            let values: Vec<String> = r.filter_map(|v| v.0.ok()).collect();
            let len = values.len() - 1;

            values.iter().enumerate().for_each(|(n, v)| {
                self.write(v);
                if n != len {
                    self.sep(&separator);
                }
            });

            self.end_line();
            pb.inc(1);
        });

        pb.finish_with_message(format!("written {size_hint} rows"));
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
