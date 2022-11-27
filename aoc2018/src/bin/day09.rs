use std::collections::VecDeque;

struct State {
    marbles: VecDeque<usize>,
    players: Vec<usize>,

    turn: usize,
    max_turns: usize,
}

impl State {
    fn new(n_players: usize, n_marbles: usize) -> Self {
        let mut marbles = VecDeque::with_capacity(n_marbles);
        marbles.push_back(0);

        let players: Vec<usize> = (0..n_players).map(|_| 0).collect();

        Self {
            marbles,
            players,
            turn: 0,
            max_turns: n_marbles,
        }
    }

    fn player(&self) -> usize {
        self.turn % self.players.len()
    }

    fn step(&mut self) {
        let player = self.player();

        self.turn += 1;

        if self.turn % 23 == 0 {
            self.marbles.rotate_right(7);
            self.players[player] += self.turn + self.marbles.pop_back().unwrap();
            self.marbles.rotate_left(1);
        } else {
            self.marbles.rotate_left(1);
            self.marbles.push_back(self.turn);
        }
    }

    fn simulate(mut self) -> usize {
        for _ in 0..self.max_turns {
            self.step();
        }

        *self.players.iter().max().unwrap_or(&0)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut marbles = Vec::new();
        for m in self.marbles.iter() {
            marbles.push(format!("{:2}", m));
        }

        write!(
            f,
            "{:6} [{:4}] {}",
            self.turn,
            self.player(),
            marbles.join(" ")
        )
    }
}

fn main() {
    let n_players = 466;
    let n_marbles = 71436;

    println!("Part 1: {}", State::new(n_players, n_marbles).simulate());
    println!(
        "Part 2: {}",
        State::new(n_players, n_marbles * 100).simulate()
    );
}
