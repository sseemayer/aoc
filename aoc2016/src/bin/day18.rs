use anyhow::Result;

fn generate(start_row: &[bool], rows: usize) -> usize {
    let width = start_row.len();
    let mut last_row = start_row.to_vec();
    let mut new_row = Vec::with_capacity(width);
    let mut n_safe = start_row.iter().filter(|v| !*v).count();

    for _ in 1..rows {
        for j in 0..width {
            let left = if j > 0 { last_row[j - 1] } else { false };
            let center = last_row[j];
            let right = if j < width - 1 {
                last_row[j + 1]
            } else {
                false
            };

            let current = match (left, center, right) {
                (true, true, false) => true,
                (false, true, true) => true,
                (true, false, false) => true,
                (false, false, true) => true,
                _ => false,
            };

            if !current {
                n_safe += 1;
            }

            new_row.push(current);
        }

        last_row = new_row;
        new_row = Vec::with_capacity(width);
    }

    n_safe
}

fn main() -> Result<()> {
    let first_row: Vec<bool> = std::fs::read_to_string("data/day18/input")?
        .trim()
        .chars()
        .map(|c| c == '^')
        .collect();

    println!("Part 1: Got {} safe tiles", generate(&first_row, 40));
    println!("Part 2: Got {} safe tiles", generate(&first_row, 400000));

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
