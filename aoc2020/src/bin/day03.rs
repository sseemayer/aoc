use std::fs::File;

use snafu::{ResultExt, Snafu};

use aoc2020::map::{Map, MapError, ParseMapTile};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Map error: {}", source))]
    MapLoading { source: MapError },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Tree,
    PathEmpty,
    PathTree,
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Tree),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Tree => "#",
                Tile::PathEmpty => "O",
                Tile::PathTree => "X",
            }
        )
    }
}

fn count_trees(map: &mut Map<[usize; 2], Tile>, di: usize, dj: usize) -> usize {
    let (_, [imax, jmax]) = map.get_extent();
    let mut i = 0;
    let mut j = 0;
    let mut hit_trees = 0;

    while i <= imax {
        let current = { map.get(&[i, j]).cloned() };

        map.set(
            [i, j],
            match current {
                None => Tile::PathEmpty,
                Some(Tile::Tree) => {
                    hit_trees += 1;
                    Tile::PathTree
                }
                _ => unreachable!(),
            },
        );

        i += di;
        j = (j + dj) % (jmax + 1);
    }

    hit_trees
}

fn main() -> Result<()> {
    let filename = "data/day03/input";
    let mut f = File::open(filename).context(Io {
        filename: filename.to_string(),
    })?;

    let map = Map::<[usize; 2], Tile>::read(&mut f).context(MapLoading)?;
    let recipes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut product = 1;
    for (di, dj) in recipes {
        let mut instance = map.clone();
        let hit_trees = count_trees(&mut instance, di, dj);

        println!("==== RECIPE {} right, {} down ====", dj, di);
        println!("{}", instance);
        println!("Hit trees: {}\n", hit_trees);

        product *= hit_trees;
    }
    println!("Final answer is {}", product);

    Ok(())
}
