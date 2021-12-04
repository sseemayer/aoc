use aoc2021::io::{read_lines, ReadLinesError};

fn count_increasing(data: &[i64], window_size: usize) -> usize {
    let mut out = 0;

    let mut last_sum = 0;
    for i in window_size..data.len() {
        let mut sum = 0;
        for j in (i - window_size)..i {
            sum += data[j];
        }

        if last_sum < sum {
            out += 1;
        }

        last_sum = sum;
    }

    out
}

fn main() -> Result<(), ReadLinesError<i64>> {
    let data = read_lines("data/day01/input")?;

    let part1 = count_increasing(&data[..], 1);
    let part2 = count_increasing(&data[..], 3);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
