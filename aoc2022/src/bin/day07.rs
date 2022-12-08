use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use colored::Colorize;

#[derive(Debug, Default)]
struct Directory {
    directories: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn get_mut(&mut self, path: &[String]) -> &mut Directory {
        if path.is_empty() {
            self
        } else {
            let next = path[0].clone();
            let rest = &path[1..];
            let subdir = self.directories.entry(next).or_default();
            subdir.get_mut(rest)
        }
    }

    fn pretty_print(&self, depth: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = (0..depth).map(|_| "  ").collect::<String>();

        for (dname, dir) in self.directories.iter() {
            write!(f, "{}📁 {} [{}]\n", indent, dname.green(), dir.total_size())?;
            dir.pretty_print(depth + 1, f)?;
        }

        for (fname, size) in self.files.iter() {
            write!(f, "{}{} ({})\n", indent, fname, size)?;
        }

        Ok(())
    }

    fn total_size(&self) -> usize {
        self.directories
            .iter()
            .map(|(_, d)| d.total_size())
            .sum::<usize>()
            + self.files.iter().map(|(_, size)| size).sum::<usize>()
    }

    fn part1(&self) -> usize {
        self.directories
            .iter()
            .map(|(_, d)| {
                let ts = d.total_size();
                if ts <= 100000 {
                    ts + d.part1()
                } else {
                    d.part1()
                }
            })
            .sum::<usize>()
    }

    fn part2(&self, min_delete: usize) -> Option<usize> {
        let min_from_children = self
            .directories
            .iter()
            .filter_map(|(_, dir)| dir.part2(min_delete))
            .min();

        let ts = self.total_size();
        if let Some(mfc) = min_from_children {
            // there is a solution coming from a child -- this is always better than the current
            // level, so take that as a solution
            Some(mfc)
        } else if ts >= min_delete {
            // the current level would be a solution
            Some(ts)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(0, f)
    }
}

fn parse(path: &str) -> Result<Directory> {
    let mut out = Directory::default();

    let mut stack = Vec::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let tokens = line.trim().split_whitespace().collect::<Vec<_>>();

        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                match tokens[2] {
                    "/" => {
                        stack.clear();
                    }
                    ".." => {
                        stack.pop();
                    }
                    dirname => {
                        stack.push(dirname.to_string());
                    }
                }
            }
        } else {
            let current_dir = out.get_mut(&stack[..]);

            let name = tokens[1].to_string();
            if tokens[0] == "dir" {
                current_dir.directories.insert(name, Directory::default());
            } else {
                let size: usize = tokens[0].parse()?;
                current_dir.files.insert(name, size);
            }
        }
    }

    Ok(out)
}

fn main() -> Result<()> {
    let parsed = parse("data/day07/input")?;

    println!("{}", parsed);

    println!("Part 1: {}", parsed.part1());

    let free_space = 70_000_000 - parsed.total_size();
    let min_delete = 30_000_000 - free_space;
    println!("Part 2: {:?}", parsed.part2(min_delete));

    Ok(())
}
