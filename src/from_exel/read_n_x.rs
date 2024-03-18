use std::{fs::OpenOptions, io::BufWriter, path::PathBuf, time::Instant};

use calamine::{open_workbook_auto, Data, Range, Reader};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

use super::{operators::FileWritingOperator, CsvError, CsvRow, CsvRowOperator};

pub fn read(source: &PathBuf, target: &PathBuf, sep: &str) -> Result<String, String> {
    let started = Instant::now();
    // find source file
    let sce = PathBuf::from(source);
    match sce.extension().and_then(|s| s.to_str()) {
        Some("xlsx" | "xlsm" | "xlsb" | "xls") => (),
        _ => return Err("Expecting an excel file".to_owned()),
    }

    // open xl file
    let mut xl = open_workbook_auto(&sce).unwrap();

    println!("Workbook open after {}", HumanDuration(started.elapsed()));

    let target_path = PathBuf::from(target).with_extension("csv");
    let _target_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(target_path.clone())
        .unwrap();
    println!(
        "Created file {} after {}",
        target_path.to_str().unwrap(),
        HumanDuration(started.elapsed())
    );

    xl.worksheets().iter().for_each(|sheet| {
        let title = &sheet.0;
        let range = &sheet.1;
        // create or append to target file

        let target_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(target_path.clone())
            .unwrap();

        let target = BufWriter::new(target_file);

        println!(
            "Created file writer for {} after {}",
            title,
            HumanDuration(started.elapsed())
        );

        let res = write_range(
            range,
            FileWritingOperator { writer: target },
            sep.to_owned(),
            started,
        )
        .map_err(|err| err.0);

        match res {
            Ok(()) => {}
            Err(e) => eprint! {"{e}"},
        }
    });

    Ok("All sheets written".to_owned())
}

fn write_range(
    range: &Range<Data>,
    mut operator: impl CsvRowOperator,
    sep: String,
    started: Instant,
) -> Result<(), CsvError> {
    let spinner_style =
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}").unwrap();
    let pb = ProgressBar::new(range.get_size().0.try_into().unwrap());
    pb.set_style(spinner_style);

    let all_rows = range.rows().map(|r| {
        pb.inc(1);
        CsvRow::iterator(r)
    });
    let res = operator.operate(sep, all_rows);
    pb.finish_and_clear();
    println!("Done in {}", HumanDuration(started.elapsed()));
    res
}
