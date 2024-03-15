use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};

use calamine::{open_workbook_auto, Data, Range, Reader};

use super::{operators::FileWritingOperator, CsvError, CsvRow, CsvRowOperator};

pub fn read(source: &PathBuf, target: &PathBuf, sep: &str) -> Result<String, String> {
    // find source file
    let sce = PathBuf::from(source);
    match sce.extension().and_then(|s| s.to_str()) {
        Some("xlsx" | "xlsm" | "xlsb" | "xls") => (),
        _ => return Err("Expecting an excel file".to_owned()),
    }

    // open xl file
    let mut xl = open_workbook_auto(&sce).unwrap();
    xl.worksheets().iter().for_each(|sheet| {
        let title = &sheet.0;
        let range = &sheet.1;

        // create or append to target file
        let target_path = PathBuf::from(target).with_extension("csv");
        let target_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(target_path)
            .unwrap();
        let target = BufWriter::new(target_file);

        let res = write_range(
            range,
            FileWritingOperator { writer: target },
            sep.to_owned(),
        )
        .map(|()| format!("Done writing sheet: {title}"))
        .map_err(|err| err.0);

        match res {
            Ok(o) => println!("{o}"),
            Err(e) => eprint! {"{e}"},
        }
    });

    Ok("All Done".to_owned())
}

fn write_range(
    range: &Range<Data>,
    mut operator: impl CsvRowOperator,
    sep: String,
) -> Result<(), CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    let rows_len = all_rows.size_hint().0;
    println!("-- reading {rows_len} lines --");
    operator.operate(sep, all_rows)
}
