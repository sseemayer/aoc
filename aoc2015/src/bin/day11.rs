use aoc2015::{format_err, Result};

use std::collections::HashSet;

#[derive(Debug)]
struct Password {
    value: Vec<u8>,
    bad_letters: HashSet<u8>,
    alphabet: String,
}

impl Password {
    fn from_str(s: &str, alphabet: &str, bad_letters: &str) -> Self {
        let value: Vec<u8> = s
            .chars()
            .filter_map(|c| alphabet.find(c))
            .map(|v| v as u8)
            .collect();

        let bad_letters: HashSet<u8> = bad_letters
            .chars()
            .filter_map(|c| alphabet.find(c))
            .map(|v| v as u8)
            .collect();

        Password {
            value,
            alphabet: alphabet.to_owned(),
            bad_letters,
        }
    }

    fn inc(&mut self) -> Result<()> {
        let mut pos: i64 = (self.value.len() - 1) as i64;
        let n_alpha = self.alphabet.len() as u8;

        while pos >= 0 {
            let p = pos as usize;
            self.value[p] = (self.value[p] + 1) % n_alpha;

            if self.value[p] > 0 {
                return Ok(());
            } else {
                pos -= 1
            }
        }

        Err(format_err!("Cannot generate new password"))
    }

    fn to_str(&self) -> String {
        self.value
            .iter()
            .map(|v| {
                let v = *v as usize;
                &self.alphabet[v..(v + 1)]
            })
            .collect()
    }

    fn has_straight3(&self) -> bool {
        for i in 0..(self.value.len() - 2) {
            let a = self.value[i];
            let b = self.value[i + 1];
            let c = self.value[i + 2];

            if b == a + 1 && c == a + 2 {
                return true;
            }
        }
        false
    }

    fn has_no_bad_letters(&self) -> bool {
        for v in self.value.iter() {
            if self.bad_letters.contains(v) {
                return false;
            }
        }
        true
    }

    fn has_two_pairs(&self) -> bool {
        let mut pairs = HashSet::new();

        for i in 0..(self.value.len() - 1) {
            let a = self.value[i];
            let b = self.value[i + 1];

            if a == b {
                pairs.insert(a);
            }
        }

        pairs.len() >= 2
    }

    fn meets_policy(&self) -> bool {
        self.has_straight3() && self.has_no_bad_letters() && self.has_two_pairs()
    }
}

fn main() -> Result<()> {
    let mut pass = Password::from_str("vzbxkghb", "abcdefghijklmnopqrstuvwxyz", "iol");

    println!("Initial password: {:?}", pass);

    let mut found_passwords = 0;
    loop {
        if pass.meets_policy() {
            println!("p {}", pass.to_str());

            found_passwords += 1;

            if found_passwords >= 2 {
                break;
            }
        }

        pass.inc()?;
    }

    Ok(())
}
