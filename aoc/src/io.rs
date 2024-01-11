use anyhow::{anyhow, Result};

use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use crate::input::InputSource;

pub fn read_all<S: InputSource>(source: S) -> Result<String> {
    let mut file = source.get_input()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn read_lines<T: FromStr, S: InputSource>(source: S) -> Result<Vec<T>>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    read_lines_reader(source.get_input()?)
}

pub fn read_lines_reader<T: FromStr, R: Read>(r: R) -> Result<Vec<T>>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    read_lines_bufreader(BufReader::new(r))
}

pub fn read_lines_bufreader<T: FromStr, R: BufRead>(reader: R) -> Result<Vec<T>>
where
    <T as FromStr>::Err: std::fmt::Display,
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
        .map(|l| <T as FromStr>::from_str(&l).map_err(|e| anyhow!("Parse error: {}", e)))
        .collect::<Result<Vec<T>>>()
}
