use aoc2021::io::{read_lines, ReadLinesError};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day10Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Read(#[from] ReadLinesError<ParseResult>),
}

type Result<T> = std::result::Result<T, Day10Error>;

#[derive(Debug)]
enum ParseResult {
    Incomplete {
        reason: IncompleteReason,
        line: String,
    },
    Corrupted {
        reason: CorruptionReason,
        line: String,
    },
}

#[derive(Debug)]
struct IncompleteReason {
    stack: Vec<(usize, char)>,
}

impl IncompleteReason {
    fn score(&self) -> i64 {
        let mut score = 0;
        let mut stack = self.stack.clone();
        while let Some((_i, c)) = stack.pop() {
            score *= 5;

            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
        }
        score
    }
}

#[derive(Debug)]
enum CorruptionReason {
    Mismatch {
        opening_character: char,
        opening_position: usize,
        closing_character: char,
        closing_position: usize,
    },
    TooMuchClosing {
        closing_character: char,
        closing_position: usize,
    },
}

impl CorruptionReason {
    fn score(&self) -> i64 {
        match self {
            CorruptionReason::TooMuchClosing {
                closing_character, ..
            }
            | CorruptionReason::Mismatch {
                closing_character, ..
            } => match closing_character {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad character {} at position {}: '{}'", .character, .position, .line)]
    BadCharacter {
        character: char,
        position: usize,
        line: String,
    },
}

impl std::str::FromStr for ParseResult {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<ParseResult, ParseError> {
        let mut stack = Vec::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push((i, c));
                }
                ')' | ']' | '}' | '>' => {
                    match stack.pop() {
                        Some((j, d)) => match (d, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                            _ => {
                                return Ok(ParseResult::Corrupted {
                                    line: s.to_string(),
                                    reason: CorruptionReason::Mismatch {
                                        opening_character: d,
                                        opening_position: j,
                                        closing_character: c,
                                        closing_position: i,
                                    },
                                })
                            }
                        },
                        None => {
                            return Ok(ParseResult::Corrupted {
                                line: s.to_string(),
                                reason: CorruptionReason::TooMuchClosing {
                                    closing_character: c,
                                    closing_position: i,
                                },
                            });
                        }
                    };
                }
                _ => {
                    return Err(ParseError::BadCharacter {
                        character: c,
                        position: i,
                        line: s.to_string(),
                    });
                }
            }
        }

        // finished reading in characters

        Ok(ParseResult::Incomplete {
            line: s.to_string(),
            reason: IncompleteReason { stack },
        })
    }
}

fn main() -> Result<()> {
    let lines = read_lines("data/day10/input")?;

    let syntax_score = lines
        .iter()
        .filter_map(|result| {
            if let ParseResult::Corrupted { reason, .. } = result {
                Some(reason.score())
            } else {
                None
            }
        })
        .sum::<i64>();

    let mut autocomplete_scores: Vec<i64> = lines
        .iter()
        .filter_map(|result| {
            if let ParseResult::Incomplete { reason, .. } = result {
                Some(reason.score())
            } else {
                None
            }
        })
        .collect();

    autocomplete_scores.sort();

    let middle_score = autocomplete_scores[autocomplete_scores.len() / 2];

    println!("Part 1: {}", syntax_score);
    println!("Part 2: {}", middle_score);

    Ok(())
}
