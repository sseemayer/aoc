use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
enum Instruction {
    /// swap position X with position Y - means that the letters at indexes X and Y (counting from 0) should be swapped.
    SwapPosition { x: usize, y: usize },

    /// swap letter X with letter Y - means that the letters X and Y should be swapped (regardless of where they appear in the string).
    SwapLetter { x: char, y: char },

    /// rotate left X steps - means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
    RotateLeft { steps: usize },

    /// rotate right X steps - means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
    RotateRight { steps: usize },

    /// rotate based on position of letter X - means that the whole string should be rotated to the right based on the index of letter X (counting from 0) as determined before this instruction does any rotations. Once the index is determined, rotate the string to the right one time, plus a number of times equal to that index, plus one additional time if the index was at least 4.
    RotateBasedOnLetterPosition { x: char },

    /// reverse positions X through Y - means that the span of letters at indexes X through Y (including the letters at X and Y) should be reversed in order.
    Reverse { x: usize, y: usize },

    /// move position X to position Y - means that the letter which is at index X should be removed from the string, then inserted such that it ends up at index Y.
    Move { x: usize, y: usize },
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Ok(match &tokens[..] {
            &["swap", "position", x, "with", "position", y] => {
                let x = x.parse().context("Parse swap first pos")?;
                let y = y.parse().context("Parse swap second pos")?;
                Instruction::SwapPosition { x, y }
            }
            &["swap", "letter", x, "with", "letter", y] => {
                let x = x.chars().next().unwrap();
                let y = y.chars().next().unwrap();
                Instruction::SwapLetter { x, y }
            }
            &["rotate", "left", steps, _] => {
                let steps = steps.parse().context("Parse rotate left steps")?;
                Instruction::RotateLeft { steps }
            }
            &["rotate", "right", steps, _] => {
                let steps = steps.parse().context("Parse rotate right steps")?;
                Instruction::RotateRight { steps }
            }
            &["rotate", "based", "on", "position", "of", "letter", x] => {
                let x = x.chars().next().unwrap();
                Instruction::RotateBasedOnLetterPosition { x }
            }
            &["reverse", "positions", x, "through", y] => {
                let x = x.parse().context("Parse reverse first pos")?;
                let y = y.parse().context("Parse reverse second pos")?;
                Instruction::Reverse { x, y }
            }
            &["move", "position", x, "to", "position", y] => {
                let x = x.parse().context("Parse move first pos")?;
                let y = y.parse().context("Parse move second pos")?;
                Instruction::Move { x, y }
            }
            _ => return Err(anyhow!("Bad instruction: '{}'", s)),
        })
    }
}

impl Instruction {
    fn undo(&self, data: Vec<char>) -> Vec<char> {
        match *self {
            Instruction::SwapPosition { x, y } => Instruction::SwapPosition { x, y }.process(data),
            Instruction::SwapLetter { x, y } => Instruction::SwapLetter { x, y }.process(data),
            Instruction::RotateLeft { steps } => Instruction::RotateRight { steps }.process(data),
            Instruction::RotateRight { steps } => Instruction::RotateLeft { steps }.process(data),
            Instruction::RotateBasedOnLetterPosition { x } => {
                for steps in 0..data.len() {
                    let reversed = Instruction::RotateLeft { steps }.process(data.clone());
                    let applied =
                        Instruction::RotateBasedOnLetterPosition { x }.process(reversed.clone());
                    if applied == data {
                        return reversed;
                    }
                }

                panic!("Cannot reverse")
            }
            Instruction::Reverse { x, y } => Instruction::Reverse { x, y }.process(data),
            Instruction::Move { x, y } => {
                //    x  y
                //    |  |
                //  abCdefghi
                //       |
                //  abdefCghi
                //
                //
                //  reverse:
                //    x  y
                //    |  |
                //  abdefCghi
                //    |
                //  abCdefghi
                Instruction::Move { x: y, y: x }.process(data)
            }
        }
    }

    fn process(&self, mut data: Vec<char>) -> Vec<char> {
        match self {
            Instruction::SwapPosition { x, y } => {
                let s = data[*x];
                data[*x] = data[*y];
                data[*y] = s;
            }
            Instruction::SwapLetter { x, y } => {
                for a in data.iter_mut() {
                    if *a == *x {
                        *a = *y
                    } else if *a == *y {
                        *a = *x
                    }
                }
            }
            Instruction::RotateLeft { steps } => {
                for _ in 0..*steps {
                    let v = data.remove(0);
                    data.push(v);
                }
            }
            Instruction::RotateRight { steps } => {
                for _ in 0..*steps {
                    let v = data.pop().unwrap();
                    data.insert(0, v);
                }
            }
            Instruction::RotateBasedOnLetterPosition { x } => {
                let i = data
                    .iter()
                    .enumerate()
                    .find_map(|(i, a)| if a == x { Some(i) } else { None })
                    .unwrap();

                let n_rotations = 1 + i + if i >= 4 { 1 } else { 0 };
                for _ in 0..n_rotations {
                    let v = data.pop().unwrap();
                    data.insert(0, v);
                }
            }
            Instruction::Reverse { x, y } => {
                data[*x..*y + 1].reverse();
            }
            Instruction::Move { x, y } => {
                let v = data.remove(*x);
                data.insert(*y, v);
            }
        }

        data
    }
}

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn main() -> Result<()> {
    let mut instructions: Vec<Instruction> = std::fs::read_to_string("data/day21/input")?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut data = chars("abcdefgh");
    for inst in &instructions {
        data = inst.process(data);
    }
    println!("Part 1: {}", data.iter().collect::<String>());

    let mut data = chars("fbgdceah");
    instructions.reverse();
    for inst in &instructions {
        data = inst.undo(data);
    }
    println!("Part 2: {}", data.iter().collect::<String>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_position() {
        // swap position 4 with position 0 swaps the first and last letters,
        // producing the input for the next step, ebcda.
        assert_eq!(
            Instruction::SwapPosition { x: 4, y: 0 }.process(chars("abcde")),
            chars("ebcda")
        );
        assert_eq!(
            Instruction::SwapPosition { x: 4, y: 0 }.undo(chars("ebcda",)),
            chars("abcde"),
        );
    }

    #[test]
    fn test_swap_letter() {
        // swap letter d with letter b swaps the positions of d and b: edcba.
        assert_eq!(
            Instruction::SwapLetter { x: 'd', y: 'b' }.process(chars("ebcda")),
            chars("edcba")
        );
        assert_eq!(
            Instruction::SwapLetter { x: 'd', y: 'b' }.undo(chars("edcba")),
            chars("ebcda"),
        );
    }

    #[test]
    fn test_reverse_positions() {
        // reverse positions 0 through 4 causes the entire string to be reversed, producing abcde.
        assert_eq!(
            Instruction::Reverse { x: 0, y: 4 }.process(chars("edcba")),
            chars("abcde")
        );
        assert_eq!(
            Instruction::Reverse { x: 0, y: 4 }.undo(chars("abcde")),
            chars("edcba"),
        );
    }

    #[test]
    fn test_rotate_left() {
        // rotate left 1 step shifts all letters left one position,
        // causing the first letter to wrap to the end of the string: bcdea
        assert_eq!(
            Instruction::RotateLeft { steps: 1 }.process(chars("abcde")),
            chars("bcdea")
        );

        assert_eq!(
            Instruction::RotateLeft { steps: 1 }.undo(chars("bcdea")),
            chars("abcde")
        );
    }

    #[test]
    fn test_move() {
        // move position 1 to position 4 removes the letter at position 1 (c),
        // then inserts it at position 4 (the end of the string): bdeac.
        assert_eq!(
            Instruction::Move { x: 1, y: 4 }.process(chars("bcdea")),
            chars("bdeac")
        );
        assert_eq!(
            Instruction::Move { x: 1, y: 4 }.undo(chars("bdeac")),
            chars("bcdea")
        );

        // move position 3 to position 0 removes the letter at position 3 (a),
        // then inserts it at position 0 (the front of the string): abdec
        assert_eq!(
            Instruction::Move { x: 3, y: 0 }.process(chars("bdeac")),
            chars("abdec")
        );
        assert_eq!(
            Instruction::Move { x: 3, y: 0 }.undo(chars("abdec")),
            chars("bdeac")
        );
    }

    #[test]
    fn test_rotate_based_on_letter() {
        // rotate based on position of letter b finds the index of letter b (1),
        // then rotates the string right once plus a number of times equal to that index (2): ecabd.
        assert_eq!(
            Instruction::RotateBasedOnLetterPosition { x: 'b' }.process(chars("abdec")),
            chars("ecabd")
        );

        assert_eq!(
            Instruction::RotateBasedOnLetterPosition { x: 'b' }.undo(chars("ecabd")),
            chars("abdec")
        );

        // rotate based on position of letter d finds the index of letter d (4),
        // then rotates the string right once, plus a number of times equal to that index,
        // plus an additional time because the index was at least 4, for a total of 6 right rotations: decab.
        assert_eq!(
            Instruction::RotateBasedOnLetterPosition { x: 'd' }.process(chars("ecabd")),
            chars("decab")
        );
        assert_eq!(
            Instruction::RotateBasedOnLetterPosition { x: 'd' }.undo(chars("decab")),
            chars("ecabd")
        );
    }
}
