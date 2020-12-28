use failure::Error;
type Result<T> = std::result::Result<T, Error>;

fn has_sorted_digits(v: usize) -> bool {
    let mut last_digit = v % 10;
    let mut v = v / 10;
    while v > 0 {
        let new_digit = v % 10;
        if new_digit > last_digit {
            return false;
        }
        last_digit = new_digit;
        v /= 10;
    }

    true
}

fn has_stretch(v: usize, min_length: usize, max_length: usize) -> bool {
    let mut cur_stretch = v % 10;
    let mut stretch_length = 1;
    let mut v = v / 10;

    while v > 0 {
        let new_digit = v % 10;
        if new_digit == cur_stretch {
            stretch_length += 1;
        } else {
            if (min_length <= stretch_length) && (stretch_length <= max_length) {
                return true;
            }
            cur_stretch = new_digit;
            stretch_length = 1;
        }
        v /= 10;
    }

    if (min_length <= stretch_length) && (stretch_length <= max_length) {
        return true;
    }

    false
}

fn main() -> Result<()> {
    let low = 153517;
    let high = 630395;

    let mut count1 = 0;
    let mut count2 = 0;
    for i in low..high {
        if has_sorted_digits(i) && has_stretch(i, 2, 1000) {
            count1 += 1;
        }

        if has_sorted_digits(i) && has_stretch(i, 2, 2) {
            count2 += 1;
        }
    }

    println!("Part 1: Got {} numbers", count1);
    println!("Part 2: Got {} numbers", count2);

    println!("t: {}", has_stretch(114444, 2, 2));

    Ok(())
}
