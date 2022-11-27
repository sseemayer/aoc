use std::{fs::File, io::Read};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self
                .children
                .iter()
                .map(|c| c.sum_metadata())
                .sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            self.metadata
                .iter()
                .filter_map(|m| self.children.get(*m - 1))
                .map(|c| c.value())
                .sum::<usize>()
        }
    }
}

fn parse(f: &str) -> Result<Node> {
    let mut data = String::new();
    File::open(f)?.read_to_string(&mut data)?;

    let data = data
        .split_whitespace()
        .map(|v| v.parse::<usize>().context("token parsing"))
        .collect::<Result<Vec<usize>>>()?;

    let (out, rest) = tokens_to_tree(&data[..])?;

    if !rest.is_empty() {
        bail!("Returned with non-empty root");
    }

    Ok(out)
}

fn tokens_to_tree(data: &[usize]) -> Result<(Node, &[usize])> {
    if data.len() < 2 {
        bail!("Called with only {} tokens: {:?}", data.len(), data);
    }

    let n_children = data[0];
    let n_metadata = data[1];

    let mut rest = &data[2..];

    let mut children = Vec::new();

    for _ in 0..n_children {
        let (child, new_rest) = tokens_to_tree(rest)?;
        children.push(child);
        rest = new_rest;
    }

    let metadata = rest[..n_metadata].to_vec();

    rest = &rest[n_metadata..];

    Ok((Node { children, metadata }, rest))
}

fn main() -> Result<()> {
    let tree = parse("data/day08/input")?;

    println!("Part 1: {}", tree.sum_metadata());
    println!("Part 2: {}", tree.value());

    Ok(())
}
