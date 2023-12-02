use anyhow::{Context, Result};

fn run1(mut offsets: Vec<i64>) -> usize {
    let mut ic: i64 = 0;
    let mut steps = 0;
    while ic >= 0 && (ic as usize) < offsets.len() {
        let ofs = offsets.get_mut(ic as usize).unwrap();
        ic += *ofs;
        *ofs += 1;
        steps += 1;
    }
    steps
}

fn run2(mut offsets: Vec<i64>) -> usize {
    let mut ic: i64 = 0;
    let mut steps = 0;
    while ic >= 0 && (ic as usize) < offsets.len() {
        let ofs = offsets.get_mut(ic as usize).unwrap();
        ic += *ofs;

        if *ofs >= 3 {
            *ofs -= 1;
        } else {
            *ofs += 1;
        }
        steps += 1;
    }
    steps
}

fn main() -> Result<()> {
    let offsets: Vec<i64> = std::fs::read_to_string("data/day05/input")?
        .lines()
        .map(|l| l.parse().context("Parse input ints"))
        .collect::<Result<_>>()?;

    println!("Part 1: {}", run1(offsets.clone()));
    println!("Part 2: {}", run2(offsets.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
