use aoc2015::Result;

fn main() -> Result<()> {
    let target_row = 3010;
    let target_col = 3019;

    let factor = 252533;
    let modulus = 33554393;

    let mut row = 1;
    let mut col = 1;
    let mut val: usize = 20151125;
    let mut progress = 1;

    while (row != target_row) || (col != target_col) {
        //println!("{:5} {:5}: {}", row, col, val);
        val = (val * factor) % modulus;

        row -= 1;
        col += 1;

        if row == 0 {
            progress += 1;
            row = progress;
            col = 1;
        }
    }
    println!("{:5} {:5}: {}", row, col, val);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
}
