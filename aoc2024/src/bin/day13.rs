use anyhow::{Context, Result};
use nalgebra::{Matrix2, Vector2};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_BUTTON: Regex =
        Regex::new(r"Button (A|B):\s*X\+(\d+),\s*Y\+(\d+)").expect("valid regex");
    static ref RE_PRIZE: Regex = Regex::new(r"Prize:\s*X=(\d+),\s*Y=(\d+)").expect("valid regex");
}

#[derive(Debug, Clone)]
struct Game {
    a: Vector2<f64>,
    b: Vector2<f64>,
    x: Vector2<f64>,
}

impl Game {
    fn solve(&self) -> Option<i64> {
        // n * a + m * b = x

        let m = Matrix2::from_columns(&[self.a, self.b]);

        let minv = m.try_inverse()?;

        let sol = minv * self.x;
        let sol_round = sol.map(|v| v.round() as i64);

        if (sol - sol_round.cast::<f64>()).norm() < 1.0e-3 {
            let [n, m] = sol_round.as_slice() else {
                return None;
            };

            Some(3 * n + m)
        } else {
            None
        }
    }
}

fn parse(data: &str) -> Result<Vec<Game>> {
    let mut a = Vector2::new(0., 0.);
    let mut b = Vector2::new(0., 0.);

    let mut out = Vec::new();
    for line in data.lines() {
        let line = line.trim();

        if let Some(m_button) = RE_BUTTON.captures(line) {
            let letter = m_button.get(1).unwrap().as_str();
            let x: f64 = m_button
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .context("Parse X coord")?;
            let y: f64 = m_button
                .get(3)
                .unwrap()
                .as_str()
                .parse()
                .context("Parse Y coord")?;

            match letter {
                "A" => {
                    a = Vector2::new(x, y);
                }
                "B" => {
                    b = Vector2::new(x, y);
                }
                _ => panic!("Weird letter"),
            }
        } else if let Some(m_prize) = RE_PRIZE.captures(line) {
            let x: f64 = m_prize
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .context("Parse X coord")?;
            let y: f64 = m_prize
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .context("Parse Y coord")?;

            out.push(Game {
                a: a.clone(),
                b: b.clone(),
                x: Vector2::new(x, y),
            });
        }
    }

    Ok(out)
}

fn main() -> Result<()> {
    let games = parse(&aoc::io::read_all((2024, 13))?)?;
    //let games = parse(&aoc::io::read_all("data/day13/example")?)?;

    let sum = games.iter().filter_map(|g| g.solve()).sum::<i64>();

    println!("Part 1: {}", sum);

    let games2: Vec<Game> = games
        .iter()
        .map(|g| {
            let mut g = g.clone();
            g.x += Vector2::from_element(10000000000000.);
            g
        })
        .collect();

    let sum2 = games2.iter().filter_map(|g| g.solve()).sum::<i64>();

    println!("Part 2: {}", sum2);
    Ok(())
}
