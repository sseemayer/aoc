use failure::Error;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct System {
    parent: HashMap<String, String>,
}

impl System {
    fn get_ancestry(&self, id: &str) -> Vec<String> {
        let mut par = self.parent.get(id);
        let mut out = Vec::new();
        while let Some(p) = par {
            out.push(p.to_owned());
            par = self.parent.get(p);
        }

        out
    }
}

impl std::iter::FromIterator<String> for System {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut out = System {
            parent: HashMap::new(),
        };

        for line in iter {
            let tokens: Vec<_> = line.split(")").collect();

            let a = tokens[0].to_owned();
            let b = tokens[1].to_owned();

            // b)a:  b orbits a
            out.parent.insert(b, a);
        }

        out
    }
}

fn main() -> Result<()> {
    let f = File::open("data/day06/input")?;
    let br = BufReader::new(f);

    let system: System = br
        .lines()
        .filter_map(|v| Result::ok(v.map_err(|e| e.into())))
        .collect();

    let res: usize = system
        .parent
        .keys()
        .map(|k| system.get_ancestry(k).len())
        .sum();

    println!("Got {} orbits", res);

    let anc_you: HashSet<_> = system.get_ancestry("YOU").into_iter().collect();
    let anc_san: HashSet<_> = system.get_ancestry("SAN").into_iter().collect();

    let pth: HashSet<_> = anc_you.symmetric_difference(&anc_san).collect();

    println!("Orbital transfers: {}", pth.len());

    Ok(())
}
