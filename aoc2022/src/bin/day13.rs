use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Number(n) => write!(f, "{}", n),
            Item::List(l) => {
                write!(
                    f,
                    "[{}]",
                    l.iter()
                        .map(|i| format!("{}", i))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

impl TryFrom<&[char]> for Item {
    type Error = anyhow::Error;

    fn try_from(chars: &[char]) -> Result<Self, Self::Error> {
        let mut items = Vec::new();
        let mut stack = Vec::new();
        let mut last_pos = 0;
        for (i, &c) in chars.into_iter().enumerate() {
            match c {
                '[' => {
                    // open
                    stack.push(i);
                }
                ']' => {
                    // close
                    let open_pos = stack.pop().ok_or_else(|| anyhow!("Matching open paren"))?;

                    if stack.is_empty() {
                        let item: Item = chars[open_pos + 1..i].try_into()?;
                        items.push(item);
                        last_pos = i + 1;
                    }
                }
                ',' if stack.is_empty() => {
                    // list
                    if i > last_pos {
                        let n = chars[last_pos..i].into_iter().collect::<String>();
                        let n: i32 = n.parse()?;
                        items.push(Item::Number(n));
                    }

                    last_pos = i + 1;
                }
                _ => {}
            }
        }

        if last_pos < chars.len() {
            let n = chars[last_pos..].into_iter().collect::<String>();
            let n: i32 = n.parse()?;
            items.push(Item::Number(n));
        }

        Ok(Item::List(items))
    }
}

impl std::str::FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().collect::<Vec<char>>()[1..s.len() - 1].try_into()
    }
}

impl std::cmp::PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Number(n), Item::Number(m)) => i32::cmp(n, m),
            (Item::Number(_), Item::List(_)) => Self::cmp(&Item::List(vec![self.clone()]), other),
            (Item::List(_), Item::Number(_)) => Self::cmp(self, &Item::List(vec![other.clone()])),
            (Item::List(k), Item::List(l)) => {
                for (n, m) in k.iter().zip(l.iter()) {
                    match Self::cmp(n, m) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }

                usize::cmp(&k.len(), &l.len())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Pair(Item, Item);

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n", self.0, self.1)
    }
}

fn parse(path: &str) -> Result<Vec<Pair>> {
    let lines = BufReader::new(File::open(path)?)
        .lines()
        .map(|l| Ok(l?.trim().to_string()))
        .collect::<Result<Vec<String>>>()?;

    let mut pairs = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        let a: Item = lines[i].parse()?;
        let b: Item = lines[i + 1].parse()?;

        pairs.push(Pair(a, b));
    }

    Ok(pairs)
}

fn main() -> Result<()> {
    let pairs = parse("data/day13/input")?;

    let mut sum_idx = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let cmp = Item::cmp(&pair.0, &pair.1);
        println!("Pair #{} {:?}:\n{}", i, cmp, pair);

        if cmp == std::cmp::Ordering::Less {
            sum_idx += i + 1;
        }
    }

    println!("Part 1: {}\n", sum_idx);

    let mut packets = pairs
        .into_iter()
        .map(|p| vec![p.0, p.1].into_iter())
        .flatten()
        .collect::<Vec<Item>>();

    let p: Item = "[[2]]".parse()?;
    let q: Item = "[[6]]".parse()?;

    packets.push(p.clone());
    packets.push(q.clone());

    packets.sort();

    let mut ip = 0;
    let mut iq = 0;
    for (i, packet) in packets.iter().enumerate() {
        println!("{}", packet);

        if packet == &p {
            ip = i + 1
        }
        if packet == &q {
            iq = i + 1
        }
    }

    let decoder_key = ip * iq;
    println!("\nPart 2: {}", decoder_key);

    Ok(())
}
