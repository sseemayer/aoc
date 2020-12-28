use bit_set::BitSet;
use snafu::{ResultExt, Snafu};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },
}

type Result<T> = std::result::Result<T, Error>;

enum GroupMode {
    Union,
    Intersection,
}

impl GroupMode {
    fn combine(&self, mut group: BitSet, questions: BitSet) -> BitSet {
        match self {
            GroupMode::Union => {
                group.union_with(&questions);
                group
            }
            GroupMode::Intersection => {
                group.intersect_with(&questions);
                group
            }
        }
    }
}

fn parse_groups<F: Read>(f: &mut F, group_mode: GroupMode) -> Result<Vec<BitSet>> {
    let mut out = Vec::new();
    let mut group = None;
    for line in BufReader::new(f).lines() {
        let line = line.context(Io)?;

        // complete a group when encountering empty line
        if line.trim().len() == 0 {
            if let Some(grp) = group {
                out.push(grp);
                group = None;
            }
            continue;
        }

        let line_questions: BitSet = line
            .chars()
            .map(|c| (c as usize) - ('a' as usize))
            .collect();

        group = if let Some(grp) = group {
            Some(group_mode.combine(grp, line_questions))
        } else {
            Some(line_questions)
        };
    }

    // complete final group
    if let Some(grp) = group {
        out.push(grp);
    }

    Ok(out)
}

fn main() -> Result<()> {
    let mut f = File::open("data/day06/input").context(Io)?;
    let groups_1 = parse_groups(&mut f, GroupMode::Union)?;

    let mut f = File::open("data/day06/input").context(Io)?;
    let groups_2 = parse_groups(&mut f, GroupMode::Intersection)?;

    println!(
        "Question count {} union, {} intersection",
        groups_1.iter().map(|g| g.len()).sum::<usize>(),
        groups_2.iter().map(|g| g.len()).sum::<usize>(),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parsing() {
        let data = "aabbc\nadg\n\nfh";
        assert_eq!(
            parse_groups(&mut data.as_bytes(), GroupMode::Union).unwrap(),
            vec![
                BitSet::from_bytes(&[0b11110010]),
                BitSet::from_bytes(&[0b00000101]),
            ]
        );
        assert_eq!(
            parse_groups(&mut data.as_bytes(), GroupMode::Intersection).unwrap(),
            vec![
                BitSet::from_bytes(&[0b10000000]),
                BitSet::from_bytes(&[0b00000101]),
            ]
        );
    }
}
