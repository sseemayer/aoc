use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Layer {
    depth: usize,
    range: usize,
}

impl std::str::FromStr for Layer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (depth, range) = s.split_once(": ").ok_or(anyhow!("Bad line: '{}'", s))?;

        let depth = depth.parse().context("Parse depth")?;
        let range = range.parse().context("Parse range")?;

        Ok(Layer { depth, range })
    }
}

fn calculate_severity(layers: &[Layer], delay: usize) -> usize {
    let mut out = 0;
    for layer in layers {
        if (layer.depth + delay) % (layer.range * 2 - 2) == 0 {
            // collision
            out += layer.depth * layer.range;
        }
    }

    out
}

fn caught(layers: &[Layer], delay: usize) -> bool {
    layers
        .iter()
        .any(|l| (l.depth + delay) % (l.range * 2 - 2) == 0)
}

fn main() -> Result<()> {
    let layers: Vec<Layer> = aoc::io::read_lines("data/day13/input")?;

    println!("Part 1: {}", calculate_severity(&layers[..], 0));

    let mut delay = 0;
    while caught(&layers[..], delay) {
        delay += 1;
    }

    println!("Part 2: {}", delay);

    Ok(())
}
