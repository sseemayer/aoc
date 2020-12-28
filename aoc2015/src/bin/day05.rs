use aoc2015::util::read_to_lines;
use aoc2015::Result;

fn nice_vowels(s: &str) -> bool {
    let mut vowels = 0;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => {}
        }
    }

    vowels >= 3
}

fn nice_repetition(s: &str) -> bool {
    let mut chars = s.chars();
    let mut last = chars.next().unwrap();

    for a in chars {
        if a == last {
            return true;
        }
        last = a;
    }

    false
}

const BAD_WORDS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn nice_no_bad_words(s: &str) -> bool {
    for bw in BAD_WORDS.iter() {
        if s.contains(bw) {
            return false;
        }
    }

    true
}

fn nice1(s: &str) -> bool {
    nice_vowels(s) && nice_repetition(s) && nice_no_bad_words(s)
}

fn nice_repeat_pair(s: &str) -> bool {
    for i in 0..(s.len() - 1) {
        let ab = &s[i..(i + 2)];

        for j in (i + 2)..(s.len() - 1) {
            let bc = &s[j..(j + 2)];

            if ab == bc {
                return true;
            }
        }
    }

    false
}

fn nice_gap_letter(s: &str) -> bool {
    for i in 0..(s.len() - 2) {
        let a = &s[i..(i + 1)];
        let b = &s[(i + 2)..(i + 3)];

        if a == b {
            return true;
        }
    }
    false
}

fn nice2(s: &str) -> bool {
    nice_repeat_pair(s) && nice_gap_letter(s)
}

fn main() -> Result<()> {
    let strings = read_to_lines("data/day05/input")?;

    let nice_strings1: Vec<String> = strings
        .iter()
        .filter(|s| nice1(s))
        .map(|s| s.to_owned())
        .collect();

    println!("Part 1: Got {} nice strings", nice_strings1.len());

    let nice_strings2: Vec<String> = strings
        .iter()
        .filter(|s| nice2(s))
        .map(|s| s.to_owned())
        .collect();

    println!("Part 2: Got {} nice strings", nice_strings2.len());
    Ok(())
}
