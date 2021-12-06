use aoc2021::io::read_all;

use thiserror::Error;

#[derive(Error, Debug)]
enum Day06Error {
    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Default, Debug)]
struct State {
    fish: [u64; 9],
}

impl State {
    fn step(&mut self) {
        let birthing = self.fish[0];
        self.fish.rotate_left(1);
        self.fish[6] += birthing;
    }
}

fn main() -> Result<(), Day06Error> {
    let mut state = State::default();
    read_all("data/day06/input")?
        .trim()
        .split(",")
        .try_for_each(|v| {
            let v: usize = v.parse()?;
            state.fish[v] += 1;
            Result::<(), Day06Error>::Ok(())
        })?;

    for i in 0..=256 {
        println!(
            "{}:\t{:?}\t total {}",
            i,
            state.fish,
            state.fish.iter().sum::<u64>()
        );
        state.step();
    }

    Ok(())
}
