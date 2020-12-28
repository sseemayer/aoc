use aoc2015::Result;

fn look_say(s: &[u8]) -> Vec<u8> {
    let mut current: Option<u8> = None;
    let mut count = 0;
    let mut out = Vec::new();
    for d in s {
        match current {
            Some(e) => {
                if e == *d {
                    count += 1;
                } else {
                    out.push(count);
                    out.push(e);

                    count = 1;
                    current = Some(*d);
                }
            }
            None => {
                count = 1;
                current = Some(*d);
            }
        }
    }
    out.push(count);
    out.push(current.unwrap());

    out
}

fn to_str(s: &[u8]) -> String {
    s.iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join("")
}

fn to_digits(v: &usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut v = *v;

    while v > 0 {
        let digit = v % 10;
        v /= 10;

        out.insert(0, digit as u8);
    }

    out
}

fn main() -> Result<()> {
    let tests: Vec<Vec<u8>> = [1, 11, 21, 1211, 111221].iter().map(to_digits).collect();

    for test in tests {
        println!("{} -> {}", to_str(&test), to_str(&look_say(&test)));
    }

    let mut val = to_digits(&1113122113);
    for i in 0..50 {
        let val_new = look_say(&val);
        println!("after {} steps: len={}", i + 1, val_new.len());
        val = val_new;
    }

    Ok(())
}
