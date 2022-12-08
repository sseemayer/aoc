use std::{collections::HashSet, fs::File, io::Read};

use anyhow::Result;

fn parse(path: &str) -> Result<Vec<char>> {
    let mut out = String::new();
    File::open(path)?.read_to_string(&mut out)?;
    Ok(out.chars().collect())
}

fn find_unique(buf: &[char], window_size: usize) -> Option<usize> {
    for i in 0..buf.len() - window_size {
        let window = &buf[i..i + window_size];
        let unique_chars = window.iter().collect::<HashSet<_>>();

        if unique_chars.len() == window_size {
            return Some(i + window_size);
        }
    }

    None
}

fn main() -> Result<()> {
    let data = parse("data/day06/input")?;

    println!("Part 1: {}", find_unique(&data[..], 4).expect("Find start"));
    println!(
        "Part 2: {}",
        find_unique(&data[..], 14).expect("Find message")
    );
    Ok(())
}
