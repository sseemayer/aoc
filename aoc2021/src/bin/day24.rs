use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

#[derive(Debug)]
struct Block {
    check: i64,
    offset: i64,
}

fn parse_blocks(path: &str) -> Result<Vec<Block>> {
    let mut out = Vec::new();

    let mut file = File::open(path)?;
    let reader = BufReader::new(&mut file);

    let mut has_block = false;
    let mut check = 0;
    let mut offset = 0;

    let mut line_in_block = 0;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        line_in_block += 1;

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if line.starts_with("inp w") {
            if has_block {
                out.push(Block { check, offset });
            }

            has_block = true;
            line_in_block = 0;
        } else if line.starts_with("add x ") && line_in_block == 5 {
            check = tokens[2].parse()?;
        } else if line.starts_with("add y ") && line_in_block == 15 {
            offset = tokens[2].parse()?;
        }
    }

    out.push(Block { check, offset });

    Ok(out)
}

#[derive(Debug)]
struct Rule {
    pos_i: usize,
    pos_j: usize,
    delta: i64,
}

impl Rule {
    fn max(&self) -> (i64, i64) {
        if self.delta > 0 {
            (9 - self.delta, 9)
        } else {
            (9, 9 + self.delta)
        }
    }

    fn min(&self) -> (i64, i64) {
        if self.delta > 0 {
            (1, 1 + self.delta)
        } else {
            (1 - self.delta, 1)
        }
    }
}

fn blocks_to_rules(blocks: &[Block]) -> Vec<Rule> {
    let mut stack = Vec::new();
    let mut out = Vec::new();
    for (pos_j, blk) in blocks.into_iter().enumerate() {
        if blk.check > 0 {
            stack.push((pos_j, blk.offset));
        } else {
            let (pos_i, ofs) = stack.pop().expect("Pop the stack");
            // d[j] == d[i] + ofs + blk.check

            out.push(Rule {
                pos_i,
                pos_j,
                delta: ofs + blk.check,
            })
        }
    }

    out
}

fn solve<F>(rules: &[Rule], f: F) -> Vec<i64>
where
    F: Fn(&Rule) -> (i64, i64),
{
    let mut out = vec![0; 14];
    for rule in rules {
        let (v_i, v_j) = f(rule);
        out[rule.pos_i] = v_i;
        out[rule.pos_j] = v_j;
    }

    out
}

fn main() -> Result<()> {
    let blocks = parse_blocks("data/day24/input")?;
    let rules = blocks_to_rules(&blocks);

    let part1 = solve(&rules, Rule::max);

    println!(
        "Part 1: {}",
        part1.iter().map(|v| format!("{}", v)).collect::<String>()
    );

    let part2 = solve(&rules, Rule::min);

    println!(
        "Part 2: {}",
        part2.iter().map(|v| format!("{}", v)).collect::<String>()
    );
    Ok(())
}
