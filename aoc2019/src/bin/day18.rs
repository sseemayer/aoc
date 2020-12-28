use aoc2019::board::{Board, Direction, Position};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_lines;
use bit_set::BitSet;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Unknown,
    Start,
    Empty,
    Wall,
    Key { id: u8 },
    Door { id: u8 },
}

impl Tile {
    fn can_move(&self, keys: &BitSet<u32>) -> bool {
        match self {
            Tile::Unknown => false,
            Tile::Start => true,
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Key { .. } => true,
            Tile::Door { id } => keys.contains(*id as usize),
        }
    }
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl std::convert::From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Tile::Start,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'a'..='z' => Tile::Key {
                id: (c as u8) - 97u8,
            },
            'A'..='Z' => Tile::Door {
                id: (c as u8) - 65u8,
            },
            _ => Tile::Unknown,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "░".to_owned(),
            Tile::Start => "@".to_owned(),
            Tile::Empty => " ".to_owned(),
            Tile::Wall => "█".to_owned(),
            Tile::Key { id } => format!("{}", (97u8 + id) as char),
            Tile::Door { id } => format!("{}", (65u8 + id) as char),
        };
        write!(f, "{}", c)
    }
}

fn parse_board(lines: &Vec<String>) -> Result<Board<Tile>> {
    let mut board = Board::new();

    for (i, l) in lines.into_iter().enumerate() {
        for (j, t) in l.chars().enumerate() {
            let tile: Tile = t.into();
            let pos = Position {
                i: i as i64,
                j: j as i64,
            };

            board.set(&pos, tile);
        }
    }

    Ok(board)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct PosMarker(usize);

fn all_pair_shortest_path(
    board: &Board<Tile>,
    pois: &HashMap<PosMarker, Position>,
) -> HashMap<PosMarker, HashMap<PosMarker, (usize, BitSet<u32>, BitSet<u32>)>> {
    let mut out: HashMap<PosMarker, HashMap<PosMarker, (usize, BitSet<u32>, BitSet<u32>)>> =
        HashMap::new();

    for (mi, i) in pois {
        let i = *i;

        let mut hist: HashMap<Position, Vec<Position>> = HashMap::new();
        let mut queue: Vec<Position> = vec![i];
        hist.insert(i, Vec::new());

        while !queue.is_empty() {
            let p = queue.remove(0);
            let h = hist[&p].clone();

            for dir in Direction::ALL.iter() {
                let p2 = p + dir.to_ofs().into();
                let t2 = board.get(&p2);

                if t2 == Tile::Wall || t2 == Tile::Unknown {
                    continue;
                }

                // we can go from p to p2 - only do so if we haven't been there yet
                if !hist.contains_key(&p2) {
                    let mut h2 = h.clone();
                    h2.push(p2);

                    hist.insert(p2, h2);
                    queue.push(p2);
                }
            }
        }

        let mut adj: HashMap<PosMarker, (usize, BitSet<u32>, BitSet<u32>)> = HashMap::new();

        for (mj, j) in pois {
            if !hist.contains_key(j) {
                continue;
            }
            let j = *j;
            let h = hist[&j].clone();
            let mut needed_keys: BitSet<u32> = BitSet::new();
            let mut encountered_keys: BitSet<u32> = BitSet::new();

            for p in &h {
                if let Tile::Door { id } = board.get(p) {
                    needed_keys.insert(id as usize);
                } else if let Tile::Key { id } = board.get(p) {
                    encountered_keys.insert(id as usize);
                }
            }

            adj.insert(*mj, (h.len(), needed_keys, encountered_keys));
        }

        out.insert(*mi, adj);
    }
    out
}

fn find_keys(board: &Board<Tile>) -> HashMap<u8, Position> {
    let mut out: HashMap<u8, Position> = HashMap::new();

    for (p, t) in board.tiles.iter() {
        if let Tile::Key { id } = t {
            out.insert(*id, *p);
        }
    }

    out
}

type BfsPos = (SmallVec<[PosMarker; 4]>, BitSet<u32>);

#[derive(Eq, PartialEq)]
struct QueueItem {
    dist: usize,
    pos: BfsPos,
    // hist: Vec<SmallVec<[PosMarker; 4]>>,
}

impl std::cmp::PartialOrd for QueueItem {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&rhs.dist)
    }
}

impl std::cmp::Ord for QueueItem {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&rhs.dist)
    }
}

fn bfs_keyseq(
    //board: &Board<Tile>,
    start_pos: &Vec<PosMarker>,
    keys: &HashMap<u8, PosMarker>,
    apsp: &HashMap<PosMarker, HashMap<PosMarker, (usize, BitSet<u32>, BitSet<u32>)>>,
) -> Option<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut seen: HashSet<BfsPos> = HashSet::new();
    let mut queue: BinaryHeap<Reverse<QueueItem>> = BinaryHeap::new();

    let start_pos = (start_pos.iter().map(|p| *p).collect(), BitSet::new());
    queue.push(Reverse(QueueItem {
        dist: 0,
        pos: start_pos.clone(),
        //hist: Vec::new(),
    }));

    let mut max_found_keys = 0;

    while !queue.is_empty() {
        let Reverse(QueueItem {
            dist: d,
            pos: (p1, k1),
            //hist: h1,
        }) = queue.pop().unwrap();

        print!("\rQueue: {:6} d={:6}       ", queue.len(), d);

        if k1.len() > max_found_keys {
            max_found_keys = k1.len();
            println!("\n{} / {} keys in {} steps", k1.len(), keys.len(), d);

            if k1.len() == keys.len() {
                return Some(d); //h1);
            }
        }

        seen.insert((p1.clone(), k1.clone()));

        'next_key: for (next_key, pk) in keys.iter() {
            // don't visit the same key again
            if k1.contains(*next_key as usize) {
                continue;
            }

            let which_robot = start_pos
                .0
                .iter()
                .enumerate()
                .find_map(|(i, p)| {
                    if let Some(_) = apsp[p].get(pk) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .unwrap();

            let (pathlen, needed_keys, encountered_keys) = &apsp[&p1[which_robot]][pk];

            let mut p2 = p1.clone();
            p2[which_robot] = *pk;

            // check that we can get this key with our current keyring
            for nk in needed_keys.iter() {
                if !k1.contains(nk) {
                    continue 'next_key;
                }
            }

            // check that we don't cross other keys without picking them up
            for ek in encountered_keys.iter() {
                if ek != *next_key as usize && !k1.contains(ek) {
                    continue 'next_key;
                }
            }

            //let mut h2 = h1.clone();
            //h2.push(p2.clone());

            let mut k2 = k1.clone();
            k2.insert(*next_key as usize);

            // don't revisit already-seen nodes
            if seen.contains(&(p2.clone(), k2.clone())) {
                continue;
            }

            queue.push(Reverse(QueueItem {
                dist: d + pathlen,
                pos: (p2.clone(), k2),
                //    hist: h2,
            }));
        }
    }

    None
}

fn format_keyseq(
    board: &Board<Tile>,
    pois: &HashMap<PosMarker, Position>,
    start_pos: &Vec<PosMarker>,
    apsp: &HashMap<PosMarker, HashMap<PosMarker, (usize, BitSet<u32>, BitSet<u32>)>>,
    ks: &Vec<SmallVec<[PosMarker; 4]>>,
) -> String {
    let mut keyseq: Vec<String> = Vec::new();
    keyseq.push("@".to_owned());
    let mut cur = start_pos.clone();
    let mut sumdist = 0;
    for next in ks.iter() {
        let changed = next
            .iter()
            .zip(cur.iter())
            .enumerate()
            .find_map(|(i, (n, c))| if n != c { Some(i) } else { None })
            .unwrap();
        let t = board.get(&pois[&next[changed]]);
        let dist = &apsp[&cur[changed]][&next[changed]].0;
        sumdist += dist;
        keyseq.push(format!(" -{}-> {}", dist, t));

        cur = next.to_vec();
    }

    format!("{} ({} steps total)", keyseq.join(""), sumdist)
}

fn run(board: &Board<Tile>) {
    println!("Board:\n{}", board);

    let start_pos = board.where_are(&Tile::Start);
    println!("Start: {:?}", start_pos);

    let keys = find_keys(&board);
    println!("Keys: {:#?}", keys);

    let mut pois: HashMap<PosMarker, Position> = HashMap::new();

    let mut key_markers: HashMap<u8, PosMarker> = HashMap::new();
    for (k, kp) in &keys {
        let pm = PosMarker(pois.len());
        pois.insert(pm, *kp);
        key_markers.insert(*k, pm);
    }

    let mut start_pos_markers = Vec::new();
    for sp in &start_pos {
        let pm = PosMarker(pois.len());
        pois.insert(pm, *sp);
        start_pos_markers.push(pm);
    }

    let apsp = all_pair_shortest_path(&board, &pois);

    // for (i, row) in apsp.iter() {
    //     for (j, (p, k, e)) in row.iter() {
    //         let ti = board.get(i);
    //         let tj = board.get(j);
    //         println!(
    //             "{} -> {}: {} steps, need_keys: {:?} gets_keys: {:?}",
    //             ti,
    //             tj,
    //             p.len(),
    //             k,
    //             e
    //         );
    //     }
    // }

    if let Some(res) = bfs_keyseq(&start_pos_markers, &key_markers, &apsp) {
        println!("Got there in {} steps", res);
        //println!(
        //    "Solution: {}",
        //    format_keyseq(&board, &pois, &start_pos_markers, &apsp, &res),
        //);
    }
}

fn main() -> Result<()> {
    let mut board = parse_board(&read_to_lines("data/day18/input")?)?;

    //println!("PART 1:");
    //run(&board);

    board.set(&Position { i: 39, j: 39 }, Tile::Start);
    board.set(&Position { i: 39, j: 40 }, Tile::Wall);
    board.set(&Position { i: 39, j: 41 }, Tile::Start);
    board.set(&Position { i: 40, j: 39 }, Tile::Wall);
    board.set(&Position { i: 40, j: 40 }, Tile::Wall);
    board.set(&Position { i: 40, j: 41 }, Tile::Wall);
    board.set(&Position { i: 41, j: 39 }, Tile::Start);
    board.set(&Position { i: 41, j: 40 }, Tile::Wall);
    board.set(&Position { i: 41, j: 41 }, Tile::Start);

    run(&board);

    Ok(())
}
