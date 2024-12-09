use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
struct Memory {
    banks: Vec<Bank>,
}

impl Memory {
    fn checksum(&self) -> usize {
        let mut sum = 0;
        let mut pos = 0;
        for bank in &self.banks {
            if let Some(id) = bank.id {
                for _ in 0..bank.length {
                    sum += pos * id;
                    pos += 1;
                }
            } else {
                pos += bank.length;
            }
        }

        sum
    }

    fn defrag1(&mut self) -> Result<()> {
        if self.banks.is_empty() {
            return Ok(());
        }

        let mut i = 0;
        let mut j = self.banks.len() - 1;

        while i < j {
            // println!("i={}, j={}\t{}", i, j, self);

            // move j pointer left until encountering a non-empty bank
            while i < j && self.banks[j].id.is_none() {
                j -= 1;
            }

            // take out data from that bank
            let Bank {
                id: Some(id),
                mut length,
            } = self.banks[j]
            else {
                continue;
            };
            self.banks[j].id = None;

            // println!("moving bank with id {} and length {}", id, length);

            // keep looking for space for the data
            while length > 0 {
                // skip to next free bank of memory
                while self.banks[i].id.is_some() {
                    i += 1;
                }

                if self.banks[i].length <= length {
                    // println!("replace {:?} at: {}", &self.banks[i], i);
                    self.banks[i].id = Some(id);
                    length -= self.banks[i].length;
                } else {
                    // println!("split {:?} at: {}", &self.banks[i], i);
                    let new_bank = Bank {
                        id: None,
                        length: self.banks[i].length - length,
                    };

                    self.banks[i].id = Some(id);
                    self.banks[i].length = length;
                    length = 0;

                    // println!("new bank {:?} at: {}", &new_bank, i + 1);

                    self.banks.insert(i + 1, new_bank);

                    i += 1;
                }

                // println!("{} remaining", length);
            }
        }

        Ok(())
    }

    fn defrag2(&mut self) -> Result<()> {
        if self.banks.is_empty() {
            return Ok(());
        }

        let mut max_id = self.banks.iter().filter_map(|b| b.id).max().unwrap_or(0) + 1;

        for j in (0..self.banks.len()).rev() {
            let Bank {
                id: Some(id),
                length,
            } = self.banks[j]
            else {
                continue;
            };

            if id >= max_id {
                continue;
            }

            max_id = usize::min(id, max_id);

            self.banks[j].id = None;

            // println!("Moving bank {} with id {} length {}", j, id, length);

            // scan for place to insert
            for i in 0..=j {
                if let Bank {
                    id: None,
                    length: space_length,
                } = self.banks[i]
                {
                    if space_length == length {
                        // println!("place bank {} with length {} at position {}", id, length, i);
                        self.banks[i].id = Some(id);
                        break;
                    } else if space_length > length {
                        // println!("split position {} with length {} into bank with length {} and new bank with length {}", i, space_length, length, space_length - length);
                        let new_bank = Bank {
                            id: None,
                            length: space_length - length,
                        };

                        self.banks[i].id = Some(id);
                        self.banks[i].length = length;
                        self.banks.insert(i + 1, new_bank);
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bank in &self.banks {
            if f.alternate() {
                write!(f, "{:#}\n", bank)?;
            } else {
                write!(f, "{}", bank)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Bank {
    id: Option<usize>,
    length: usize,
}

impl std::fmt::Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.id {
            if f.alternate() {
                write!(f, "{}x{}", self.length, id)?;
            } else {
                for _ in 0..self.length {
                    write!(f, "{}", id)?;
                }
            }
        } else {
            if f.alternate() {
                write!(f, "{}x<empty>", self.length)?;
            } else {
                for _ in 0..self.length {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for Memory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut banks = Vec::new();
        let mut id = 0;
        let mut in_file = true;
        for c in s.trim().chars() {
            let length = c.to_digit(10).ok_or(anyhow!("cannot parse digit: {}", c))? as usize;

            if in_file {
                banks.push(Bank {
                    id: Some(id),
                    length,
                });
                id += 1;
                in_file = false;
            } else {
                banks.push(Bank { id: None, length });
                in_file = true;
            }
        }

        Ok(Self { banks })
    }
}

fn part1(mem: &Memory) -> Result<()> {
    let mut mem = mem.clone();
    mem.defrag1()?;

    // println!("{}", mem);

    println!("Part 1: {}", mem.checksum());

    Ok(())
}

fn part2(mem: &Memory) -> Result<()> {
    let mut mem = mem.clone();
    mem.defrag2()?;

    // println!("{}", mem);

    println!("Part 2: {}", mem.checksum());

    Ok(())
}

fn main() -> Result<()> {
    let mem: Memory = aoc::io::read_all((2024, 09))?.parse()?;
    //let mem: Memory = aoc::io::read_all("data/day09/example")?.parse()?;

    // println!("{}", mem);

    part1(&mem)?;
    part2(&mem)?;

    Ok(())
}
