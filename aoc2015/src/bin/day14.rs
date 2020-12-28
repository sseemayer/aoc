use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_duration: usize,
    rest_duration: usize,
}

impl std::str::FromStr for Reindeer {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // 0      1   2   3  4    5   6 7        8   9    10   11   12  13  14
        // Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
        let tokens: Vec<&str> = s.split(" ").collect();

        if tokens.len() != 15 {
            return Err(format_err!("Wrong number of tokens: '{}'", s));
        }

        let name = tokens[0].to_owned();
        let speed: usize = tokens[3].parse()?;
        let fly_duration: usize = tokens[6].parse()?;
        let rest_duration: usize = tokens[13].parse()?;

        Ok(Reindeer {
            name,
            speed,
            fly_duration,
            rest_duration,
        })
    }
}

#[derive(Debug)]
enum ReindeerState {
    Flying { duration: usize, total: usize },
    Resting { duration: usize, total: usize },
}

impl std::fmt::Display for ReindeerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ReindeerState::Flying { duration, total } => {
                write!(f, "Flying  ({:4} / {:4})", duration, total)
            }
            ReindeerState::Resting { duration, total } => {
                write!(f, "Resting ({:4} / {:4})", duration, total)
            }
        }
    }
}

#[derive(Debug)]
struct SimState {
    reindeer: Reindeer,
    state: ReindeerState,
    distance_traveled: usize,
    score: usize,
}

impl SimState {
    fn step(&mut self) {
        match self.state {
            ReindeerState::Flying {
                ref mut duration,
                total,
            } => {
                if *duration < total {
                    *duration += 1;
                    self.distance_traveled += self.reindeer.speed;
                } else {
                    self.state = ReindeerState::Resting {
                        duration: 1,
                        total: self.reindeer.rest_duration,
                    }
                }
            }
            ReindeerState::Resting {
                ref mut duration,
                total,
            } => {
                if *duration < total {
                    *duration += 1;
                } else {
                    self.distance_traveled += self.reindeer.speed;
                    self.state = ReindeerState::Flying {
                        duration: 1,
                        total: self.reindeer.fly_duration,
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for SimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{:8}: {} d={:6} s={:4}",
            self.reindeer.name, self.state, self.distance_traveled, self.score
        )
    }
}

fn main() -> Result<()> {
    let reindeers: Vec<Reindeer> = read_to_parsed_lines("data/day14/input", &|l| l.parse())?;

    let mut states: Vec<SimState> = reindeers
        .iter()
        .map(|r| SimState {
            reindeer: r.clone(),
            state: ReindeerState::Flying {
                duration: 0,
                total: r.fly_duration,
            },
            distance_traveled: 0,
            score: 0,
        })
        .collect();

    for i in 1..=2503 {
        println!("=== AFTER {:5} SECONDS ===", i);
        for s in states.iter_mut() {
            s.step();
            println!("{}", s);
        }

        let mut max_dist = 0;
        for s in states.iter() {
            if s.distance_traveled > max_dist {
                max_dist = s.distance_traveled;
            }
        }

        for s in states.iter_mut() {
            if s.distance_traveled == max_dist {
                s.score += 1;
            }
        }
    }

    println!("=== FINAL RANKING BY SCORE ===");
    states.sort_by_key(|r| std::cmp::Reverse(r.distance_traveled));
    for (i, s) in states.iter_mut().enumerate() {
        s.step();
        println!("#{}, {}", i + 1, s);
    }

    println!("=== FINAL RANKING BY SCORE ===");
    states.sort_by_key(|r| std::cmp::Reverse(r.score));
    for (i, s) in states.iter_mut().enumerate() {
        s.step();
        println!("#{}, {}", i + 1, s);
    }

    Ok(())
}
