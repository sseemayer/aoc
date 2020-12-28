use regex::Regex;
use snafu::{ResultExt, Snafu};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int parsing error: {}", source))]
    ParseInt { source: std::num::ParseIntError },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct BagDefinition {
    container_to_contents: HashMap<String, Vec<(usize, String)>>,
    content_to_containers: HashMap<String, Vec<String>>,
}

impl BagDefinition {
    pub fn read<F: Read>(f: &mut F) -> Result<Self> {
        let re_line = Regex::new(r"([a-z ]+) bags contain (.*).").unwrap();
        let re_content = Regex::new(r"(\d)+ ([a-z ]+) bags?").unwrap();

        let br = BufReader::new(f);

        let mut container_to_contents = HashMap::new();
        let mut content_to_containers = HashMap::new();

        for line in br.lines() {
            let line = line.context(Io)?;
            if line.is_empty() {
                continue;
            }

            let captures = re_line.captures(&line).unwrap();
            let color = captures.get(1).unwrap().as_str();
            let contents = captures.get(2).unwrap().as_str();
            let contents = if contents == "no other bags" {
                Vec::new()
            } else {
                contents
                    .split(", ")
                    .map(|c| {
                        let caps = re_content.captures(c).unwrap();
                        let count: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                        let content = caps.get(2).unwrap().as_str();
                        (count, content.to_string())
                    })
                    .collect()
            };

            for (_, content_color) in &contents {
                content_to_containers
                    .entry(content_color.to_string())
                    .or_insert_with(Vec::new)
                    .push(color.to_string());
            }
            container_to_contents.insert(color.to_string(), contents);
        }

        Ok(BagDefinition {
            container_to_contents,
            content_to_containers,
        })
    }

    fn get_all_containers(&self, color: &str) -> HashSet<String> {
        let mut out = HashSet::new();

        fn rec(color: &str, ctc: &HashMap<String, Vec<String>>, mut out: &mut HashSet<String>) {
            let containers = ctc.get(color);
            if let Some(cntnrs) = containers {
                //println!("color {} -> {:?}", color, containers);
                for cntnr in cntnrs {
                    out.insert(cntnr.clone());
                    rec(&cntnr, &ctc, &mut out);
                }
            } else {
                //println!("color {} -> []", color);
            }
        }

        rec(color, &self.content_to_containers, &mut out);
        out
    }

    fn get_all_contents(&self, color: &str) -> Vec<(usize, String)> {
        let mut out = Vec::new();

        fn rec(
            factor: usize,
            color: &str,
            ctc: &HashMap<String, Vec<(usize, String)>>,
            mut out: &mut Vec<(usize, String)>,
        ) {
            let contents = ctc.get(color);
            if let Some(cntnts) = contents {
                for (count, inner_color) in cntnts {
                    out.push((count * factor, inner_color.to_string()));
                    rec(count * factor, inner_color, ctc, &mut out);
                }
            }
        }

        rec(1, color, &self.container_to_contents, &mut out);

        out
    }
}

fn main() -> Result<()> {
    let mut f = File::open("data/day07/input").context(Io)?;
    let bags = BagDefinition::read(&mut f)?;

    let parents = bags.get_all_containers("shiny gold");
    println!(
        "{} containers eventually contain shiny gold bags",
        parents.len()
    );

    let children = bags.get_all_contents("shiny gold");
    println!(
        "shiny gold bags eventually contain {} bags",
        children.iter().map(|(c, _)| c).sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parsing() {
        let mut f = File::open("data/day07/example").unwrap();
        let bags = BagDefinition::read(&mut f).unwrap();
        println!("{:#?}", bags);

        assert_eq!(bags.container_to_contents.len(), 9);
        assert_eq!(
            bags.container_to_contents["light red"],
            vec![
                (1, "bright white".to_string()),
                (2, "muted yellow".to_string()),
            ]
        );
        assert_eq!(
            bags.container_to_contents["dark orange"],
            vec![
                (3, "bright white".to_string()),
                (4, "muted yellow".to_string())
            ]
        );
        assert_eq!(bags.container_to_contents["faded blue"], vec![]);

        assert_eq!(
            bags.content_to_containers["shiny gold"],
            vec!["bright white".to_string(), "muted yellow".to_string()]
        );

        assert_eq!(
            bags.get_all_containers("shiny gold"),
            vec!["bright white", "muted yellow", "dark orange", "light red"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        );

        assert_eq!(
            bags.get_all_contents("shiny gold"),
            vec![
                (1, "dark olive".to_string()),
                (3, "faded blue".to_string()),
                (4, "dotted black".to_string()),
                (2, "vibrant plum".to_string()),
                (10, "faded blue".to_string()),
                (12, "dotted black".to_string())
            ]
        )
    }
}
