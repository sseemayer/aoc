use anyhow::Result;

fn get_power_level(x: usize, y: usize, grid_serial_number: usize) -> i8 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;

    // Begin with a power level of the rack ID times the Y coordinate.
    // Increase the power level by the value of the grid serial number (your puzzle input).
    let power_level_initial = rack_id * y + grid_serial_number;

    // Set the power level to itself multiplied by the rack ID.
    let power_level_mid = power_level_initial * rack_id;

    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    let hundreds_digit = ((power_level_mid - power_level_mid % 100) / 100) % 10;

    // Subtract 5 from the power level.
    hundreds_digit as i8 - 5
}

fn find_top(
    grid_serial_number: usize,
    grid_size: usize,
    window_size_max: usize,
) -> (usize, usize, usize) {
    let mut top_power = i32::MIN;
    let mut top_power_at = (0, 0, 0);

    for window_size in 3..=window_size_max {
        for wx in 1..=grid_size - window_size {
            for wy in 1..=grid_size - window_size {
                let mut power = 0;

                for x in wx..wx + window_size {
                    for y in wy..wy + window_size {
                        power += get_power_level(x, y, grid_serial_number) as i32;
                    }
                }

                if power > top_power {
                    top_power = power;
                    top_power_at = (wx, wy, window_size);

                    println!("{} {:?}", top_power, top_power_at);
                }
            }
        }
    }

    top_power_at
}

fn main() -> Result<()> {
    println!("Part 1: {:?}", find_top(5034, 300, 3));
    println!("Part 2: {:?}", find_top(5034, 300, 300));

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(get_power_level(3, 5, 8), 4);
        assert_eq!(get_power_level(122, 79, 57), -5);
        assert_eq!(get_power_level(217, 196, 39), 0);
        assert_eq!(get_power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_top() {
        assert_eq!(find_top(18, 300, 3), (33, 45, 3));
        assert_eq!(find_top(42, 300, 3), (21, 61, 3));

        assert_eq!(find_top(18, 300, 18), (90, 269, 16));
    }
}
