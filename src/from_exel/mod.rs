use std::path::PathBuf;

use self::read_n_x::read;

mod read_n_x;

pub fn all(source: &PathBuf, target: &PathBuf, sep: &str) -> Result<String, String> {
    read(source, target, sep)
}
