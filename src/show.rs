use std::{fs::File, num::TryFromIntError, path::PathBuf};

use qwitlib::lines::read_file_lines;

pub fn csv(source: &PathBuf, num: i64) -> Result<String, String> {
    let num_sized: usize = num
        .try_into()
        .map_err(|err: TryFromIntError| err.to_string())?;
    let file = File::open(source.clone()).map_err(|err| format!("source: {source:?} {err}"))?;
    read(file, num_sized)
}

fn read(source: File, num_sized: usize) -> Result<String, String> {
    let lines = read_file_lines(source).map_err(|err| format!("{err:?}"))?;

    lines.take(num_sized).enumerate().for_each(|(n, line)| {
        if let Ok(line) = line {
            if n == 0 {
                println!("{line}");
                println!();
            } else {
                println!("{line}");
            }
        }
    });
    println!();
    Ok("_________file end_________".to_owned())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Seek, SeekFrom, Write},
    };

    use super::*;

    #[test]
    fn opening_file_that_does_exist() {
        let mut tmpfile: File = tempfile::tempfile().unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        let _ = tmpfile.write_all(
            b"hello;world
            1234;56789
            4321;98765",
        );

        let res = read(tmpfile, 10);
        assert!(res.is_ok(), "file read {res:?}");
    }
}
