use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail, Context, Error, Result};

fn char_to_state(value: char) -> Result<Option<bool>> {
    Ok(match value {
        '.' => Some(true),
        '#' => Some(false),
        '?' => None,
        _ => bail!("Bad state: '{}'", value),
    })
}

#[allow(dead_code)]
fn format_assignment(a: &[bool]) -> String {
    a.iter().map(|v| if *v { '.' } else { '#' }).collect()
}

#[allow(dead_code)]
fn format_pattern(p: &[Option<bool>]) -> String {
    p.iter()
        .map(|v| match v {
            Some(true) => '.',
            Some(false) => '#',
            None => '?',
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Row {
    pattern: Vec<Option<bool>>,
    groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pattern, groups) = s
            .split_once(" ")
            .ok_or_else(|| anyhow!("Bad row: '{}'", s))?;

        let pattern = pattern
            .chars()
            .map(|c| char_to_state(c))
            .collect::<Result<Vec<Option<bool>>>>()?;

        let groups = groups
            .split(",")
            .map(|g| g.parse().context("Parse group"))
            .collect::<Result<Vec<usize>>>()?;

        Ok(Self { pattern, groups })
    }
}

impl Row {
    fn count_assignments_recursive(
        &self,
        pattern_pos: usize,
        group_pos: usize,
        current_group_size: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(v) = cache.get(&(pattern_pos, group_pos, current_group_size)) {
            return *v;
        }

        if pattern_pos >= self.pattern.len() {
            if group_pos >= self.groups.len() {
                // we successfully consumed both the pattern and groups
                return 1;
            }

            let last_group = self.groups.len() - 1;
            if group_pos == last_group && current_group_size == self.groups[last_group] {
                // we are ending the last group with the last character
                return 1;
            }

            // there were unconsumed groups
            return 0;
        }

        let out = match self.pattern[pattern_pos] {
            Some(false) => {
                // encountering a broken spring

                if group_pos >= self.groups.len() || current_group_size >= self.groups[group_pos] {
                    // the group is becoming too big
                    return 0;
                }

                self.count_assignments_recursive(
                    pattern_pos + 1,
                    group_pos,
                    current_group_size + 1,
                    cache,
                )
            }
            Some(true) => {
                // encountering a functional spring

                if current_group_size == 0 {
                    // not currently in a group, keep going
                    self.count_assignments_recursive(pattern_pos + 1, group_pos, 0, cache)
                } else if current_group_size != self.groups[group_pos] {
                    // finished a group with incorrect size
                    0
                } else {
                    // finished a group with the correct size, on to the next
                    self.count_assignments_recursive(pattern_pos + 1, group_pos + 1, 0, cache)
                }
            }
            None => {
                // encountering a wildcard

                let mut assignments = 0;

                if current_group_size == 0 {
                    // we are not currently in a group, so a valid option is continuing not being
                    // in one
                    assignments +=
                        self.count_assignments_recursive(pattern_pos + 1, group_pos, 0, cache);
                }

                if group_pos < self.groups.len() && current_group_size < self.groups[group_pos] {
                    // we can increase the size of our current group
                    assignments += self.count_assignments_recursive(
                        pattern_pos + 1,
                        group_pos,
                        current_group_size + 1,
                        cache,
                    );
                }

                if group_pos < self.groups.len() && current_group_size == self.groups[group_pos] {
                    // we can finish a group with the appropriate size
                    assignments +=
                        self.count_assignments_recursive(pattern_pos + 1, group_pos + 1, 0, cache);
                }

                assignments
            }
        };

        cache.insert((pattern_pos, group_pos, current_group_size), out);

        out
    }

    fn count_valid_assignments(&self) -> usize {
        let mut cache = HashMap::new();
        self.count_assignments_recursive(0, 0, 0, &mut cache)
    }

    fn unfold(&self, amount: usize) -> Self {
        let mut pattern = self.pattern.clone();

        for _ in 1..amount {
            pattern.push(None);
            pattern.extend(self.pattern.iter());
        }

        let groups: Vec<usize> = (0..amount)
            .map(|_| self.groups.iter())
            .flatten()
            .cloned()
            .collect();

        Self { pattern, groups }
    }
}

fn main() -> Result<()> {
    let rows: Vec<Row> = aoc::io::read_lines("data/day12/input")?;

    println!(
        "Part 1: {}",
        rows.iter()
            .map(|r| r.count_valid_assignments())
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        rows.iter()
            .map(|r| r.unfold(5).count_valid_assignments())
            .sum::<usize>()
    );

    Ok(())
}
