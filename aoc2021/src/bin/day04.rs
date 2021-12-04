use thiserror::Error;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Error, Debug)]
enum Day04Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Day04Error>;

#[derive(Debug)]
struct Board {
    numbers: HashSet<i64>,
    lines: Vec<HashSet<i64>>,
}

fn parse_boards<R: Read>(reader: BufReader<R>) -> Result<Vec<Board>> {
    let mut board_data = Vec::new();
    let mut boards = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if !board_data.is_empty() {
                boards.push(board_data.clone());
                board_data.clear();
            }
        } else {
            let line = line
                .split_whitespace()
                .map(|t| Ok(t.parse()?))
                .collect::<Result<Vec<i64>>>()?;

            board_data.push(line);
        }
    }

    if !board_data.is_empty() {
        boards.push(board_data.clone());
    }

    let boards = boards
        .iter()
        .map(|board| {
            // turn the array into a list of horizontal and vertical winning lines
            let mut numbers = HashSet::new();
            for line in board.iter() {
                numbers.extend(line.iter());
            }

            let mut lines: Vec<HashSet<i64>> = Vec::new();

            // all horizontal lines
            for line in board.iter() {
                lines.push(line.iter().map(|v| *v).collect());
            }

            // all vertical lines
            for i in 0..board[0].len() {
                lines.push(board.iter().map(|line| line[i]).collect())
            }

            Board { numbers, lines }
        })
        .collect();

    Ok(boards)
}

impl Board {
    fn check_winning(&self, numbers: &[i64]) -> Option<HashSet<i64>> {
        let numbers: HashSet<i64> = numbers.iter().map(|n| *n).collect();
        for line in self.lines.iter() {
            if line.iter().all(|n| numbers.contains(n)) {
                return Some(self.numbers.difference(&numbers).map(|n| *n).collect());
            }
        }

        None
    }
}
fn main() -> Result<()> {
    let mut reader = BufReader::new(File::open("data/day04/input")?);

    let mut numbers = String::new();
    reader.read_line(&mut numbers)?;

    let numbers: Vec<i64> = numbers
        .split(",")
        .map(|n| Ok(n.trim().parse()?))
        .collect::<Result<Vec<i64>>>()?;

    let mut boards = parse_boards(reader)?;

    let n_boards = boards.len();
    let mut n_winning_boards = 0;

    for round in 0..numbers.len() {
        // which numbers are in play
        let selection = &numbers[0..=round];
        let last_number = numbers[round];

        // find winning boards
        let mut remove_boards = Vec::new();
        for (i, board) in boards.iter().enumerate() {
            while let Some(numbers) = board.check_winning(selection) {
                let sum: i64 = numbers.iter().sum();
                let score = last_number * sum;

                n_winning_boards += 1;

                if n_winning_boards == 1 {
                    println!("Part 1: {} * {} =  {}", sum, last_number, score);
                } else if n_winning_boards == n_boards {
                    println!("Part 2: {} * {} =  {}", sum, last_number, score);
                }

                remove_boards.push(i);

                break;
            }
        }

        remove_boards.sort_by_key(|v| boards.len() - v);
        for i in remove_boards {
            boards.remove(i);
        }
    }

    Ok(())
}
