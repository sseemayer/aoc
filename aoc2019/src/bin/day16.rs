use aoc2019::result::Result;
use aoc2019::util::read_to_string;

fn ones_place(v: i64) -> i64 {
    v.abs() % 10
}

fn transform(data: &Vec<i64>, offset: usize) -> Vec<i64> {
    let mut out = Vec::with_capacity(data.len());

    for _ in 0..offset {
        out.push(0);
    }

    if offset > data.len() / 2 {
        let mut sum: i64 = data[offset..].iter().sum();

        for i in offset..data.len() {
            out.push(sum % 10);
            sum -= data[i];
        }
    } else {
        for i in offset..data.len() {
            let mut s = 0;
            let pat_block_size = i + 1;

            let mut fact = 1;
            for block_start in (i..data.len()).step_by(pat_block_size * 2) {
                let block_end = std::cmp::min(data.len(), block_start + pat_block_size);

                for j in block_start..block_end {
                    s += data[j] * fact;
                }

                fact *= -1;
            }

            let op = ones_place(s);
            out.push(op)
        }
    }

    out
}

fn fmt_message(msg: &[i64]) -> String {
    msg.into_iter()
        .map(|d| format!("{}", d))
        .collect::<Vec<String>>()
        .join("")
}

fn get_skip_offset(msg: &[i64]) -> usize {
    let mut out = 0;

    for digit in msg {
        out = (out * 10) + digit;
    }

    out as usize
}

fn main() -> Result<()> {
    let message = read_to_string("data/day16/input")?
        .trim()
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|e: std::num::ParseIntError| e.into())
        })
        .collect::<Result<Vec<i64>>>()?;

    let skip_offset = get_skip_offset(&message[0..7]);

    let mut msg_transform1 = message.clone();
    for round in 0..100 {
        println!(
            "round {:4}: message: {}",
            round,
            fmt_message(&msg_transform1[0..8]),
        );
        msg_transform1 = transform(&msg_transform1, 0);
    }
    println!("Part 1 answer: {}", fmt_message(&msg_transform1[0..8]));
    println!("Skip offset: {}", skip_offset);

    let mut msg_transform2: Vec<i64> = message
        .iter()
        .cycle()
        .take(message.len() * 10000)
        .map(|v| *v)
        .collect();
    for round in 0..100 {
        println!(
            "round {:4}: message: {}",
            round,
            fmt_message(&msg_transform2[skip_offset..(skip_offset + 8)])
        );
        msg_transform2 = transform(&msg_transform2, skip_offset);
    }
    println!(
        "Part 2 answer: {}",
        fmt_message(&msg_transform2[skip_offset..(skip_offset + 8)])
    );

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ones_place() {
        assert_eq!(0, ones_place(0));
        assert_eq!(2, ones_place(2));
        assert_eq!(3, ones_place(13));
        assert_eq!(4, ones_place(14509704));
        assert_eq!(5, ones_place(-1235));
        assert_eq!(6, ones_place(-790709876));
    }

    #[test]
    fn test_transform1() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![4, 8, 2, 2, 6, 1, 5, 8];

        let out = transform(&data, 0);
        assert_eq!(expected, out);
    }

    #[test]
    fn test_transform2() {
        let data = vec![4, 8, 2, 2, 6, 1, 5, 8];
        let expected = vec![3, 4, 0, 4, 0, 4, 3, 8];

        let out = transform(&data, 0);
        assert_eq!(expected, out);
    }
}
