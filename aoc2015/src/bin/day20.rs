use aoc2015::Result;
use integer_sqrt::IntegerSquareRoot;

fn calc_factors(n: u64) -> Vec<u64> {
    let mut facs = Vec::new();
    let limit = n.integer_sqrt();

    for k in 1..=limit {
        if n % k == 0 {
            facs.push(k);

            let l = n / k;
            if l != k {
                facs.push(n / k);
            }
        }
    }

    facs
}

fn calc_presents_part1(facs: &[u64]) -> u64 {
    facs.iter().sum::<u64>() * 10
}

fn calc_presents_part2(facs: &[u64], house_no: u64) -> u64 {
    facs.iter().filter(|k| house_no / **k < 50).sum::<u64>() * 11
}

fn main() -> Result<()> {
    let target_gifts = 36_000_000;

    let mut house_no: u64 = 1;

    let mut found_part1 = false;
    let mut found_part2 = false;
    loop {
        let factors = calc_factors(house_no);

        let presents_part1 = calc_presents_part1(&factors);
        let presents_part2 = calc_presents_part2(&factors, house_no);

        if !found_part1 && presents_part1 >= target_gifts {
            println!("House {} meets criteria for part 1.", house_no);
            found_part1 = true;
        }

        if !found_part2 && presents_part2 >= target_gifts {
            println!("House {} meets criteria for part 2.", house_no);
            found_part2 = true;
        }

        if found_part1 && found_part2 {
            break;
        }

        house_no += 1;
    }

    Ok(())
}
