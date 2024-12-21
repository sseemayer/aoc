use std::collections::HashMap;

use anyhow::{anyhow, Result};

trait MotionWeights {
    /// gets the cost of moving from button 'from' to 'to'
    fn get_weight(&self, from: char, to: char) -> Result<usize>;
}

/// A directly-controlled keypad (i.e. zero-cost movement)
struct DirectControl;

impl MotionWeights for DirectControl {
    fn get_weight(&self, _from: char, _to: char) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone)]
struct Keypad {
    /// shortest-path connections between all buttons
    distances: HashMap<(char, char), usize>,

    /// list of valid buttons
    buttons: Vec<char>,
}

impl Keypad {
    fn from_layout(layout: &str, parent_weights: &impl MotionWeights) -> Result<Self> {
        let mut pos_to_button: HashMap<[i32; 2], char> = Default::default();
        let mut button_to_pos: HashMap<char, [i32; 2]> = Default::default();
        for (i, line) in layout.lines().enumerate() {
            for (j, button) in line.chars().enumerate() {
                if button == ' ' {
                    continue;
                }

                let pos = [i as i32, j as i32];

                button_to_pos.insert(button, pos);
                pos_to_button.insert(pos, button);
            }
        }

        let mut distances: HashMap<(char, char), usize> = Default::default();

        for (&a, &[ia, ja]) in button_to_pos.iter() {
            for (&b, &[ib, jb]) in button_to_pos.iter() {
                // go from [ia, ja] to [ib, jb]

                let di = ib - ia;
                let dj = jb - ja;

                let btn_i = if di > 0 {
                    'v'
                } else if di < 0 {
                    '^'
                } else {
                    'A'
                };
                let btn_j = if dj > 0 {
                    '>'
                } else if dj < 0 {
                    '<'
                } else {
                    'A'
                };

                let mut weight_candidates = Vec::new();

                if pos_to_button.contains_key(&[ib, ja]) {
                    // we can go vertically, then horizontally
                    // [ia, ja] -> [ib, ja] -> [ib, jb]

                    weight_candidates.push(
                        parent_weights.get_weight('A', btn_i)?
                            + (di.abs() as usize)
                            + parent_weights.get_weight(btn_i, btn_j)?
                            + (dj.abs() as usize)
                            + parent_weights.get_weight(btn_j, 'A')?,
                    );
                }

                if pos_to_button.contains_key(&[ia, jb]) {
                    // we can go horizontally, then vertically
                    // [ia, ja] -> [ia, jb] -> [ib, jb]
                    weight_candidates.push(
                        parent_weights.get_weight('A', btn_j)?
                            + (dj.abs() as usize)
                            + parent_weights.get_weight(btn_j, btn_i)?
                            + (di.abs() as usize)
                            + parent_weights.get_weight(btn_i, 'A')?,
                    );
                }

                let weight = weight_candidates
                    .into_iter()
                    .min()
                    .ok_or(anyhow!("Cannot find a path"))?;

                distances.insert((a, b), weight);
            }
        }

        let mut buttons: Vec<char> = button_to_pos.keys().cloned().collect();
        buttons.sort();

        Ok(Self { distances, buttons })
    }

    fn complexity(&self, code: &str) -> Result<usize> {
        let numeric: usize = code
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .expect("only digits");

        let sequence = self.shortest_sequence(code)?;

        Ok(numeric * sequence)
    }

    fn shortest_sequence(&self, code: &str) -> Result<usize> {
        let code: Vec<char> = code.chars().collect();

        let mut current = 'A';
        let mut sum = 0;
        for c in code {
            let weight = self.get_weight(current, c)? + 1;

            // println!("{} -> {}: {}", current, c, weight);

            sum += weight;
            current = c;
        }

        Ok(sum)
    }
}

impl std::fmt::Display for Keypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &j in &self.buttons {
            write!(f, "\t{}", j)?;
        }

        writeln!(f)?;

        for &i in &self.buttons {
            write!(f, "{}\t", i)?;
            for &j in &self.buttons {
                let v = self
                    .distances
                    .get(&(i, j))
                    .map(|d| d.to_string())
                    .unwrap_or("n/a".to_string());

                write!(f, "{}\t", v)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl MotionWeights for Keypad {
    fn get_weight(&self, from: char, to: char) -> Result<usize> {
        self.distances.get(&(from, to)).cloned().ok_or(anyhow!(
            "No connection from {} to {}",
            from,
            to
        ))
    }
}

fn make_stack(n_keypads: usize) -> Result<Keypad> {
    let mut current = Keypad::from_layout(KP_DIRECTIONAL, &DirectControl)?;

    for _ in 1..n_keypads {
        current = Keypad::from_layout(KP_DIRECTIONAL, &current)?;
    }

    Keypad::from_layout(KP_NUMERIC, &current)
}

const KP_NUMERIC: &'static str = "789\n456\n123\n 0A";
const KP_DIRECTIONAL: &'static str = " ^A\n<v>";

fn main() -> Result<()> {
    let kpad_part1 = make_stack(2)?;
    let kpad_part2 = make_stack(25)?;

    let sequences: Vec<String> = aoc::io::read_lines((2024, 21))?;
    //let sequences: Vec<String> = aoc::io::read_lines("data/day21/example")?;

    let part1 = sequences
        .iter()
        .map(|s| kpad_part1.complexity(s).expect("solvable"))
        .sum::<usize>();

    let part2 = sequences
        .iter()
        .map(|s| kpad_part2.complexity(s).expect("solvable"))
        .sum::<usize>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
