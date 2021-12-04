use thiserror::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
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
    let reader = BufReader::new(File::open(path)?);

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
