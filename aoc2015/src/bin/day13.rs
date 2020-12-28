use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};
use std::collections::{HashMap, HashSet};

use permutohedron::LexicalPermutation;

fn parse_preference(s: &str) -> Result<((String, String), i64)> {
    let tokens: Vec<String> = s.split(" ").map(|s| s.to_owned()).collect();
    if tokens.len() != 11 {
        return Err(format_err!("Invalid token count: {}", s));
    }

    let verb = &tokens[2];
    let mut score: i64 = tokens[3].parse()?;

    if verb == "lose" {
        score = -score;
    }

    let a = tokens[0].to_owned();
    let b = tokens[10].replace(".", "").to_owned();

    Ok(((a, b), score))
}

fn score_arrangement(arrangement: &Vec<String>, prefs: &HashMap<(String, String), i64>) -> i64 {
    let mut score = 0;
    for i in 0..arrangement.len() {
        let a = arrangement[i].to_owned();
        let b = arrangement[(i + 1) % arrangement.len()].to_owned();

        score += prefs.get(&(a.clone(), b.clone())).unwrap_or(&0);
        score += prefs.get(&(b.clone(), a.clone())).unwrap_or(&0);
    }
    score
}

fn find_max(arrangement: &mut Vec<String>, preferences: &HashMap<(String, String), i64>) -> i64 {
    arrangement.sort();

    let mut max_happiness = 0;
    loop {
        let happiness = score_arrangement(&arrangement, &preferences);

        if happiness > max_happiness {
            max_happiness = happiness;
        }

        if !arrangement.next_permutation() {
            break;
        }
    }

    max_happiness
}

fn main() -> Result<()> {
    let preferences: HashMap<(String, String), i64> =
        read_to_parsed_lines("data/day13/input", &|l| parse_preference(l))?;

    println!("Preferences: {:#?}", preferences);

    let mut people = HashSet::new();
    for (a, b) in preferences.keys() {
        people.insert(a.to_owned());
        people.insert(b.to_owned());
    }

    let mut arrangement: Vec<_> = people.into_iter().collect();

    println!(
        "PART ONE: Max happiness: {}",
        find_max(&mut arrangement, &preferences)
    );

    arrangement.push("Self".to_owned());
    println!(
        "PART TWO: Max happiness: {}",
        find_max(&mut arrangement, &preferences)
    );

    Ok(())
}
