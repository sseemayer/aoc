use crate::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Read a whole file, identified by a path, to a String.
pub fn read_to_string(path: &str) -> Result<String> {
    let mut f = File::open(path)?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;

    Ok(buf)
}

/// Read a whole file, identified by a path, to a list of Strings, one per line.
pub fn read_to_lines(path: &str) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let br = BufReader::new(f);

    br.lines().map(|l| Ok(l?.to_owned())).collect()
}

/// Read a whole file, identified by a path, to a list of parsed representations, one per line.
pub fn read_to_parsed_lines<
    T,
    C: std::iter::FromIterator<T>,
    F: FnOnce(&str) -> Result<T> + Copy,
>(
    path: &str,
    parser: &F,
) -> Result<C> {
    let f = File::open(path)?;
    let br = BufReader::new(f);

    br.lines().map(move |l| parser(&l?)).collect()
}
