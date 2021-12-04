use thiserror::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

#[derive(Error, Debug)]
pub enum ReadLinesError<T: FromStr>
where
    <T as FromStr>::Err: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(<T as FromStr>::Err),
}

pub fn read_lines<T: FromStr>(path: &str) -> Result<Vec<T>, ReadLinesError<T>>
where
    <T as FromStr>::Err: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    read_lines_reader(File::open(path)?)
}

pub fn read_lines_reader<T: FromStr, R: Read>(r: R) -> Result<Vec<T>, ReadLinesError<T>>
where
    <T as FromStr>::Err: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    read_lines_bufreader(BufReader::new(r))
}

pub fn read_lines_bufreader<T: FromStr, R: BufRead>(reader: R) -> Result<Vec<T>, ReadLinesError<T>>
where
    <T as FromStr>::Err: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    reader
        .lines()
        .filter_map(|l| {
            let l = l.ok()?;
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                Some(l.to_owned())
            }
        })
        .map(|l| <T as FromStr>::from_str(&l).map_err(|e| ReadLinesError::Parse(e)))
        .collect::<Result<Vec<T>, ReadLinesError<T>>>()
}
