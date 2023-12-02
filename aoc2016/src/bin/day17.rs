use std::collections::VecDeque;

use anyhow::Result;

fn get_open_doors(sequence: &str) -> (bool, bool, bool, bool) {
    let digest = *md5::compute(sequence);

    let up = (digest[0] & 0xf0) >> 4 > 0xa;
    let down = digest[0] & 0x0f > 0xa;
    let left = (digest[1] & 0xf0) >> 4 > 0xa;
    let right = digest[1] & 0x0f > 0xa;

    (up, down, left, right)
}

fn get_neighbors(i: i8, j: i8, sequence: &str) -> Vec<(i8, i8, String)> {
    let (up, down, left, right) = get_open_doors(sequence);

    let mut out = Vec::new();
    if up && i > 0 {
        out.push((i - 1, j, format!("{}U", sequence)));
    }
    if down && i < 3 {
        out.push((i + 1, j, format!("{}D", sequence)));
    }
    if left && j > 0 {
        out.push((i, j - 1, format!("{}L", sequence)));
    }
    if right && j < 3 {
        out.push((i, j + 1, format!("{}R", sequence)));
    }

    out
}

fn main() -> Result<()> {
    //let input = "ihgpwlah";
    let input = "vkjiggvb";

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, input.to_string()));

    let mut shortest_solution = None;
    let mut longest_solution = 0;

    while let Some((i, j, sequence)) = queue.pop_front() {
        if i == 3 && j == 3 {
            if shortest_solution == None {
                shortest_solution = Some(sequence.clone());
            }

            let solution_length = sequence.len() - input.len();
            if solution_length > longest_solution {
                longest_solution = solution_length;
            }
        } else {
            queue.extend(get_neighbors(i, j, &sequence));
        }
    }

    println!(
        "Part 1: {}\nPart 2: {}",
        shortest_solution.unwrap(),
        longest_solution
    );

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
