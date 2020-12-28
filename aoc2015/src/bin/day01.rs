use aoc2015::util::read_to_string;
use aoc2015::Result;

fn main() -> Result<()> {
    let data = read_to_string("data/day01/input")?;

    let mut floor = 0i64;
    let mut basement_char = None;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if floor < 0 && basement_char == None {
            basement_char = Some(i + 1);
        }
    }

    println!("Floor is {}", floor);
    println!("Character for basement is {}", basement_char.unwrap());

    Ok(())
}
