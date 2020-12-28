use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    p1: VecDeque<usize>,
    p2: VecDeque<usize>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p1: {:?}\np2: {:?}", self.p1, self.p2)
    }
}

impl State {
    fn parse<F: Read>(f: &mut F) -> Result<Self> {
        let mut p1: VecDeque<usize> = VecDeque::new();
        let mut p2: VecDeque<usize> = VecDeque::new();

        let mut target = &mut p1;

        for line in BufReader::new(f).lines() {
            let line = line.context(Io)?;
            let line = line.trim();
            if line.len() == 0 {
                continue;
            };

            if line == "Player 1:" {
                target = &mut p1;
            } else if line == "Player 2:" {
                target = &mut p2;
            } else {
                let n: usize = line.parse().context(ParseNumber {
                    data: line.to_string(),
                })?;
                target.push_back(n);
            }
        }

        Ok(State { p1, p2 })
    }

    fn score(&self) -> usize {
        let mut p1 = self.p1.clone();
        let mut p2 = self.p2.clone();

        let mut score = 0;
        let mut factor = 1;

        while !p1.is_empty() || !p2.is_empty() {
            let card = p1.pop_back().or(p2.pop_back()).unwrap();
            score += factor * card;
            factor += 1;
        }

        score
    }
}

fn indent(level: usize) -> String {
    let mut out = String::new();
    for _ in 0..level {
        out.extend("  ".chars());
    }
    out
}

#[derive(Debug, Clone)]
enum GameOutcome {
    P1Wins,
    P2Wins,
}

#[derive(Debug, Clone)]
struct Game {
    state: State,
    seen_states: HashSet<State>,
}

impl Game {
    fn from(state: &State) -> Self {
        Game {
            state: state.clone(),
            seen_states: HashSet::new(),
        }
    }

    fn play(&mut self) -> GameOutcome {
        let mut round = 0;

        loop {
            round += 1;

            if self.state.p1.is_empty() {
                return GameOutcome::P2Wins;
            } else if self.state.p2.is_empty() {
                return GameOutcome::P1Wins;
            };

            let c1 = self.state.p1.pop_front().unwrap();
            let c2 = self.state.p2.pop_front().unwrap();

            if c1 > c2 {
                self.state.p1.push_back(c1);
                self.state.p1.push_back(c2);
            } else {
                self.state.p2.push_back(c2);
                self.state.p2.push_back(c1);
            }
        }
    }

    fn play_recursive(&mut self, parents: &[usize]) -> GameOutcome {
        let mut round = 0;
        let ind = indent(parents.len());

        loop {
            round += 1;

            // println!("{}Round {:?}-{}", ind, parents, round);
            // println!("{}p1: {:?}", ind, self.state.p1);
            // println!("{}p2: {:?}", ind, self.state.p2);

            // * Before either player deals a card, if there was a previous round in this game that
            //   had exactly the same cards in the same order in the same players' decks, the game
            //   instantly ends in a win for player 1. Previous rounds from other games are not considered.
            //   (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
            if self.seen_states.contains(&self.state) {
                // println!("{}repeated!", ind);
                return GameOutcome::P1Wins;
            }
            self.seen_states.insert(self.state.clone());

            if self.state.p1.is_empty() {
                // println!("{}p1-empty!", ind);
                return GameOutcome::P2Wins;
            } else if self.state.p2.is_empty() {
                // println!("{}p2-empty!", ind);
                return GameOutcome::P1Wins;
            };

            // * Otherwise, this round's cards must be in a new configuration; the players begin the round
            //   by each drawing the top card of their deck as normal.
            let c1 = self.state.p1.pop_front().unwrap();
            let c2 = self.state.p2.pop_front().unwrap();

            // println!("{}p1: draws {}", ind, c1);
            // println!("{}p2: draws {}", ind, c2);

            // * If both players have at least as many cards remaining in their deck as the value of the
            //   card they just drew, the winner of the round is determined by playing a new game of Recursive
            //   Combat (see below).
            let outcome = if self.state.p1.len() >= c1 && self.state.p2.len() >= c2 {
                // To play a sub-game of Recursive Combat, each player creates a new deck by making a copy
                // of the next cards in their deck (the quantity of cards copied is equal to the number
                // on the card they drew to trigger the sub-game).
                // During this sub-game, the game that triggered it is on hold and completely unaffected;
                // no cards are removed from players' decks to form the sub-game. (For example, if player 1
                // drew the 3 card, their deck in the sub-game would be copies of the next three cards
                // in their deck.)
                let rec_p1 = self.state.p1.iter().take(c1).map(|c| *c).collect();
                let rec_p2 = self.state.p2.iter().take(c2).map(|c| *c).collect();

                let mut rec_game = Game {
                    state: State {
                        p1: rec_p1,
                        p2: rec_p2,
                    },
                    seen_states: HashSet::new(),
                };

                let mut rec_parents = parents.to_vec();
                rec_parents.push(round);

                rec_game.play_recursive(&rec_parents)

            // * Otherwise, at least one player must not have enough cards left in their deck to recurse;
            //   the winner of the round is the player with the higher-value card.
            } else if c1 > c2 {
                GameOutcome::P1Wins
            } else {
                GameOutcome::P2Wins
            };

            // println!("{}{:?}", ind, outcome);

            match outcome {
                GameOutcome::P1Wins => {
                    self.state.p1.push_back(c1);
                    self.state.p1.push_back(c2);
                }
                GameOutcome::P2Wins => {
                    self.state.p2.push_back(c2);
                    self.state.p2.push_back(c1);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let state = State::parse(&mut File::open("data/day22/input").context(Io)?)?;

    let mut game1 = Game::from(&state);
    game1.play();
    println!("\nPart 1: score = {}", game1.state.score());

    let mut game2 = Game::from(&state);
    game2.play_recursive(&Vec::new());
    println!("\nPart 2: score = {}", game2.state.score());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
