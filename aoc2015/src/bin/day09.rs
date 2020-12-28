use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};
use std::collections::{HashMap, HashSet};

use permutohedron::LexicalPermutation;

fn parse_connection(s: &str) -> Result<((String, String), usize)> {
    let mut tokens: Vec<String> = s.split(" ").map(|s| s.to_owned()).collect();
    if tokens.len() != 5 {
        return Err(format_err!("Invalid token count: {}", s));
    }

    let dist: usize = tokens.pop().unwrap().parse()?;

    tokens.pop(); // = sign
    tokens.remove(1); // "to"

    tokens.sort();

    let a = tokens.remove(0);
    let b = tokens.remove(0);
    Ok(((a, b), dist))
}

fn calc_dist(connections: &HashMap<(String, String), usize>, route: &[String]) -> Option<usize> {
    let mut total = 0;

    let mut iter = route.iter();
    let mut current = iter.next().unwrap();

    for next in iter {
        if let Some(d) = connections
            .get(&(current.to_string(), next.to_string()))
            .or_else(|| connections.get(&(next.to_string(), current.to_string())))
        {
            total += d;
            current = next;
        } else {
            return None;
        }
    }

    Some(total)
}

fn get_places(connections: &HashMap<(String, String), usize>) -> Vec<String> {
    let mut out: HashSet<String> = HashSet::new();
    for (a, b) in connections.keys() {
        out.insert(a.to_string());
        out.insert(b.to_string());
    }

    out.into_iter().collect()
}

fn main() -> Result<()> {
    let connections: HashMap<(String, String), usize> =
        read_to_parsed_lines("data/day09/input", &|l| parse_connection(l))?;

    let mut route = get_places(&connections);
    route.sort();

    let mut min_dist = std::usize::MAX;
    let mut max_dist = std::usize::MIN;
    let mut shortest_route = None;
    let mut longest_route = None;
    loop {
        if let Some(dist) = calc_dist(&connections, &route[..]) {
            if dist < min_dist {
                min_dist = dist;
                shortest_route = Some(route.clone());
            }

            if dist > max_dist {
                max_dist = dist;
                longest_route = Some(route.clone());
            }
        }

        if !route.next_permutation() {
            break;
        }
    }

    println!(
        "Shortest route is {:?}, distance={}",
        shortest_route.unwrap(),
        min_dist
    );

    println!(
        "Longest route is {:?}, distance={}",
        longest_route.unwrap(),
        max_dist
    );

    Ok(())
}
