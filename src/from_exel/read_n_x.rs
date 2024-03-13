use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};

use calamine::{open_workbook_auto, Data, Range, Reader};

use super::{operators::FileWritingOperator, CsvError, CsvRow, CsvRowOperator};

pub fn read(source: &PathBuf, target: &PathBuf) -> Result<String, String> {
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
    let target = BufWriter::new(target_file);

    // open xl file
    let mut xl = open_workbook_auto(&sce).unwrap();
    let range = xl.worksheet_range_at(0).unwrap().unwrap();

    write_range(&range, FileWritingOperator { writer: target })
        .map(|()| "All done".to_owned())
        .map_err(|err| err.0)
}

fn write_range(range: &Range<Data>, mut operator: impl CsvRowOperator) -> Result<(), CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    operator.operate(all_rows)
}
