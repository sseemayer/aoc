use failure::{format_err, Error};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
type Result<T> = std::result::Result<T, Error>;

struct Asteroid;

struct AsteroidField {
    data: Vec<Vec<Option<Asteroid>>>,
}

struct AsteroidIter<'a> {
    af: &'a AsteroidField,
    i: usize,
    j: usize,
}

impl<'a> Iterator for AsteroidIter<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        for i in self.i..self.af.data.len() {
            let j_start = if i == self.i { self.j } else { 0 };

            for j in j_start..self.af.data[i].len() {
                match self.af.data[i][j] {
                    Some(_) => {
                        self.i = i;
                        self.j = j + 1;
                        return Some((i, j));
                    }
                    None => {}
                }
            }
        }

        None
    }
}

fn simplify_fraction(a: i64, b: i64) -> (i64, i64) {
    let mut a = a;
    let mut b = b;

    if b == 0 {
        return (1, 0);
    }

    if a == 0 {
        if b > 0 {
            return (0, 1);
        } else {
            return (0, -1);
        }
    }

    for d in 2..=b.abs() {
        // println!("{}/{} ?/? {}", a, b, d);
        while (a % d == 0) && (b % d == 0) {
            a /= d;
            b /= d;
        }
    }

    (a, b)
}

impl AsteroidField {
    fn from_read<R: Read>(f: R) -> Result<Self> {
        let br = BufReader::new(f);

        let data: Vec<Vec<Option<Asteroid>>> = br
            .lines()
            .map(|l| {
                l?.chars()
                    .map(|c| match c {
                        '#' => Ok(Some(Asteroid)),
                        '.' => Ok(None),
                        _ => return Err(format_err!("Invalid character: '{}'", c)),
                    })
                    .collect::<Result<Vec<Option<Asteroid>>>>()
            })
            .collect::<Result<Vec<Vec<Option<Asteroid>>>>>()?;

        Ok(AsteroidField { data })
    }

    fn get(&self, i: usize, j: usize) -> Option<&Asteroid> {
        if let Some(row) = self.data.get(i) {
            if let Some(cell) = row.get(j) {
                cell.as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn can_see(&self, pos_0: (usize, usize), pos_1: (usize, usize)) -> bool {
        // ensure that i0 < i1
        let ((i0, j0), (i1, j1)) = if pos_0.0 <= pos_1.0 {
            (pos_0, pos_1)
        } else {
            (pos_1, pos_0)
        };

        let di = (i1 as i64) - (i0 as i64);
        let dj = (j1 as i64) - (j0 as i64);

        // we can only have intersections at perfect grid matches.
        // the simplified fraction corresponds to the steps to take in i and j direction until
        // another perfect grid match can be found.
        let (si, sj) = simplify_fraction(di, dj);

        // println!("Step: {} {}", si, sj);

        let mut i = (i0 as i64 + si) as usize;
        let mut j = (j0 as i64 + sj) as usize;

        let mut trace = Vec::new();
        while (i != i1) || (j != j1) {
            let has_asteroid = if let Some(_) = self.get(i, j) {
                true
            } else {
                false
            };

            if has_asteroid {
                // println!(
                //     "can_see {:?} => {:?}: collision @ ({}, {})",
                //     pos_0, pos_1, i, j
                // );
                return false;
            }

            trace.push((i, j));
            i = (i as i64 + si) as usize;
            j = (j as i64 + sj) as usize;
        }

        // println!(
        //     "can_see {:?} => {:?}: yes, s: ({}, {}) trace: {:?}",
        //     pos_0, pos_1, si, sj, trace
        // );

        true
    }

    fn get_all_visible(&self, pos_0: (usize, usize)) -> HashSet<(usize, usize)> {
        let (i0, j0) = pos_0;
        let mut visible = HashSet::new();

        for (i1, j1) in self {
            if (i0 != i1) || (j0 != j1) {
                if self.can_see((i0, j0), (i1, j1)) {
                    visible.insert((i1, j1));
                }
            }
        }

        visible
    }
}

impl<'a> IntoIterator for &'a AsteroidField {
    type Item = (usize, usize);
    type IntoIter = AsteroidIter<'a>;

    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        AsteroidIter {
            af: self,
            i: 0,
            j: 0,
        }
    }
}

impl std::fmt::Display for AsteroidField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for line in self.data.iter() {
            for a in line {
                let c = match a {
                    Some(_) => "#",
                    None => ".",
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let f = File::open("data/day10/input")?;
    let mut af = AsteroidField::from_read(f)?;

    println!("{}", af);

    let mut most_seen = 0;
    let mut most_pos = (0, 0);

    //af.can_see((13, 11), (17, 19));

    // /*

    for (i0, j0) in &af {
        let visible = af.get_all_visible((i0, j0));

        if visible.len() > most_seen {
            println!(
                "\nCan see {} asteroids from ({}, {})",
                visible.len(),
                i0,
                j0
            );

            for (i, row) in af.data.iter().enumerate() {
                for (j, a) in row.iter().enumerate() {
                    let c = match a {
                        Some(_) => {
                            if visible.contains(&(i, j)) {
                                "o"
                            } else if (i == i0) && (j == j0) {
                                "!"
                            } else {
                                "x"
                            }
                        }
                        None => ".",
                    };

                    print!("{}", c);
                }
                println!("");
            }

            most_seen = visible.len();
            most_pos = (i0, j0);
        }
    }

    println!("PART TWO");

    let (is, js) = most_pos;

    let mut zap_order = Vec::new();

    while af.into_iter().count() > 1 {
        let mut visible: Vec<_> = af.get_all_visible(most_pos).into_iter().collect();

        visible.sort_by(|p0, p1| {
            let (i0, j0) = *p0;
            let (i1, j1) = *p1;

            let di0 = (i0 as i64) - (is as i64);
            let dj0 = (j0 as i64) - (js as i64);

            let di1 = (i1 as i64) - (is as i64);
            let dj1 = (j1 as i64) - (js as i64);

            let alpha0 = (dj0 as f64).atan2(di0 as f64);
            let alpha1 = (dj1 as f64).atan2(di1 as f64);

            alpha1
                .partial_cmp(&alpha0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (iz, jz) in visible {
            zap_order.push((iz, jz));
            println!(
                "ZAP {}: ({}, {}): {}",
                zap_order.len(),
                jz,
                iz,
                jz * 100 + iz
            );
            af.data[iz][jz] = None;
        }
    }

    // */
    Ok(())
}
