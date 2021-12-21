use std::collections::HashMap;

use anyhow::Result;

#[derive(PartialEq, Eq, Hash)]
struct State {
    pos_a: u16,
    pos_b: u16,

    score_a: u16,
    score_b: u16,

    turn_a: bool,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.turn_a {
            write!(
                f,
                "[A:{:>3}@{:>2}, b:{:>3}@{:>2}]",
                self.score_a, self.pos_a, self.score_b, self.pos_b
            )
        } else {
            write!(
                f,
                "[a:{:>3}@{:>2}, B:{:>3}@{:>2}]",
                self.score_a, self.pos_a, self.score_b, self.pos_b
            )
        }
    }
}

static DIRAC_ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl State {
    fn new(pos_a: u16, pos_b: u16) -> Self {
        State {
            pos_a,
            pos_b,

            score_a: 0,
            score_b: 0,

            turn_a: true,
        }
    }

    fn step(&mut self, die: &mut u16) {
        let roll = *die * 3 + 3;

        if self.turn_a {
            self.pos_a = (self.pos_a - 1 + roll) % 10 + 1;
            self.score_a += self.pos_a;
        } else {
            self.pos_b = (self.pos_b - 1 + roll) % 10 + 1;
            self.score_b += self.pos_b;
        }

        *die = (*die - 1 + 3) % 100 + 1;
        self.turn_a = !self.turn_a;
    }

    fn dirac_step(&self) -> impl Iterator<Item = (State, usize)> + '_ {
        DIRAC_ROLLS.iter().map(|&(roll, factor)| {
            (
                if self.turn_a {
                    let pos_a = (self.pos_a - 1 + roll) % 10 + 1;
                    let score_a = self.score_a + pos_a;

                    State {
                        pos_a,
                        pos_b: self.pos_b,

                        score_a,
                        score_b: self.score_b,

                        turn_a: !self.turn_a,
                    }
                } else {
                    let pos_b = (self.pos_b - 1 + roll) % 10 + 1;
                    let score_b = self.score_b + pos_b;

                    State {
                        pos_a: self.pos_a,
                        pos_b,

                        score_a: self.score_a,
                        score_b,

                        turn_a: !self.turn_a,
                    }
                },
                factor,
            )
        })
    }
}

fn main() -> Result<()> {
    let a_pos = 6;
    let b_pos = 10;

    let mut state = State::new(a_pos, b_pos);
    let mut die = 1;
    let mut turn = 0;

    while state.score_a < 1000 && state.score_b < 1000 {
        state.step(&mut die);
        turn += 1;
    }

    println!(
        "Part 1: {}",
        u16::min(state.score_a, state.score_b) as usize * turn * 3
    );

    let mut states: HashMap<State, usize> = HashMap::new();
    states.insert(State::new(a_pos, b_pos), 1);

    let mut wins_a = 0;
    let mut wins_b = 0;
    while !states.is_empty() {
        let mut new_states: HashMap<State, usize> = HashMap::new();
        for (state, count) in states.into_iter() {
            if state.score_a >= 21 {
                wins_a += count;
            } else if state.score_b >= 21 {
                wins_b += count;
            } else {
                for (new_state, factor) in state.dirac_step() {
                    *new_states.entry(new_state).or_default() += count * factor;
                }
            }
        }

        states = new_states;
    }

    println!("Part 2: {}", usize::max(wins_a, wins_b));

    Ok(())
}
