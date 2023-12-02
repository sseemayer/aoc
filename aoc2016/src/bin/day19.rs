use std::collections::HashSet;

use anyhow::Result;

fn part1(n_elves: usize) -> usize {
    let mut elves: Vec<usize> = (1..=n_elves).collect();

    while elves.len() > 1 {
        //println!("New round! elves: {:?}", elves);
        //println!("New round with {} elves", elves.len());

        let mut bankrupt = HashSet::new();

        for i in 0..elves.len() {
            let current_elf = elves[i];
            if bankrupt.contains(&current_elf) {
                //println!("Elf {} has no presents and is skipped", current_elf);
                continue;
            }

            let right_i = (i + 1) % elves.len();
            let right_elf = elves[right_i];
            bankrupt.insert(right_elf);

            //println!("Elf {} steals from elf {}", current_elf, right_elf);
        }

        elves = elves
            .into_iter()
            .filter(|elf| !bankrupt.contains(elf))
            .collect();
    }

    elves[0]
}

fn part2(n_elves: usize) -> usize {
    let mut elves: Vec<usize> = (1..=n_elves).collect();
    while elves.len() > 1 {
        let mut next_round = Vec::with_capacity(elves.len() * 8 / 9);
        let first_high = (elves.len() + 2) / 3;
        let after_last_high = elves.len() / 2;
        let first_inc = 2 - elves.len() % 2;
        for &e in elves[first_high..after_last_high].iter() {
            next_round.push(e);
        }
        for i in (after_last_high + first_inc..elves.len()).step_by(3) {
            next_round.push(elves[i]);
        }
        for &e in elves[..first_high].iter() {
            next_round.push(e);
        }
        elves = next_round;
    }
    elves[0]
}

fn main() -> Result<()> {
    let n_elves = 3014387;

    println!("Part 1 winner: {}", part1(n_elves));
    println!("Part 2 winner: {}", part2(n_elves));

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
