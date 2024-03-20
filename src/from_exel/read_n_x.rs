use std::{fs::OpenOptions, io::BufWriter, path::PathBuf, time::Instant};

use calamine::{open_workbook_auto, Reader};
use indicatif::{HumanDuration, ProgressBar};

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

    let worksheets = xl.worksheets();

    let worksheet_amount = worksheets.len();

    let pb = ProgressBar::new(worksheet_amount.try_into().unwrap());

    for sheet in &worksheets {
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

        let res = qwitlib::from_excel::write_range(
            range,
            qwitlib::from_excel::operators::FileWritingOperator { writer: target },
            sep.to_owned(),
        )
        .map_err(|err| err.to_string());

        match res {
            Ok(()) => {}
            Err(e) => eprint! {"{e}"},
        }
        pb.inc(1);
    }

    pb.finish_with_message(format!("Done in {}", HumanDuration(started.elapsed())));
    Ok("All sheets written".to_owned())
}
