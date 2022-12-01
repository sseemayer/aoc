use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn parse(path: &str) -> Result<Vec<Vec<usize>>> {
    let mut out = Vec::new();
    let mut buf = Vec::new();

    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if !buf.is_empty() {
                out.push(buf);
            }
            buf = Vec::new();
        } else {
            let val: usize = line.parse()?;
            buf.push(val);
        }
    }

    if !buf.is_empty() {
        out.push(buf);
    }

    Ok(out)
}

fn main() -> Result<()> {
    let data = parse("data/day01/input")?;

    let mut sums = data
        .iter()
        .map(|v| v.iter().sum::<usize>())
        .collect::<Vec<_>>();

    sums.sort_by_key(|v| std::cmp::Reverse(*v));

    println!("Part 1: {}", sums.first().expect("have max"));
    println!("Part 2: {}", sums.iter().take(3).sum::<usize>());

    Ok(())
}
