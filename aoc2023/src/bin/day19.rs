use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, Clone)]
enum Operator {
    Less,
    Greater,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<(Rule, String)>,
    final_destination: String,
}

impl Workflow {
    fn apply_to(&self, part: &HashMap<String, isize>) -> &str {
        for (rule, destination) in &self.rules {
            if rule.applies_to(part) {
                return destination;
            }
        }

        &self.final_destination
    }

    fn apply_symbolically(&self, cube: &Hypercube) -> Vec<(String, Hypercube)> {
        let mut out = Vec::new();
        let mut cube = cube.clone();

        for (rule, destination) in &self.rules {
            let outcube = rule.apply_symbolically(&cube);

            if !outcube.is_empty() {
                out.push((destination.to_string(), outcube));
            }

            cube = &cube & &rule.invert().apply_symbolically(&cube);

            if cube.is_empty() {
                return out;
            }
        }

        out.push((self.final_destination.to_string(), cube));

        out
    }
}

#[derive(Debug, Clone)]
struct Rule {
    variable: String,
    operator: Operator,
    threshold: isize,
}

impl Rule {
    fn parse(s: &str) -> Result<(Rule, String)> {
        let (condition, destination) = s
            .split_once(":")
            .ok_or_else(|| anyhow!("Bad rule: '{}'", s))?;

        let destination = destination.to_string();

        let (variable, threshold, operator) =
            if let Some((variable, threshold)) = condition.split_once("<") {
                let variable = variable.to_string();
                let threshold: isize = threshold.parse().context("Parse threshold")?;

                (variable, threshold, Operator::Less)
            } else if let Some((variable, threshold)) = condition.split_once(">") {
                let variable = variable.to_string();
                let threshold: isize = threshold.parse().context("Parse threshold")?;
                (variable, threshold, Operator::Greater)
            } else {
                bail!("Bad condition: '{}'", condition)
            };

        Ok((
            Rule {
                variable,
                threshold,
                operator,
            },
            destination,
        ))
    }

    fn applies_to(&self, part: &HashMap<String, isize>) -> bool {
        if let Some(&value) = part.get(&self.variable) {
            match self.operator {
                Operator::Less => value < self.threshold,
                Operator::Greater => value > self.threshold,
            }
        } else {
            false
        }
    }

    fn invert(&self) -> Self {
        let variable = self.variable.clone();
        match self.operator {
            Operator::Less => Rule {
                variable,
                operator: Operator::Greater,
                threshold: self.threshold - 1,
            },
            Operator::Greater => Rule {
                variable,
                operator: Operator::Less,
                threshold: self.threshold + 1,
            },
        }
    }

    fn apply_symbolically(&self, cube: &Hypercube) -> Hypercube {
        let mut out = cube.clone();
        let (min, max) = out
            .dimensions
            .entry(self.variable.to_string())
            .or_insert((isize::MIN, isize::MAX));

        match self.operator {
            Operator::Less => {
                *max = isize::min(*max, self.threshold);
            }
            Operator::Greater => {
                *min = isize::max(*min, self.threshold + 1);
            }
        }

        out
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.variable,
            match self.operator {
                Operator::Less => "<",
                Operator::Greater => ">",
            },
            self.threshold
        )
    }
}

#[derive(Debug, Clone)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<HashMap<String, isize>>,
}

impl Input {
    fn parse(path: &str) -> Result<Self> {
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            } else if line.starts_with("{") {
                // parse part
                let part = line
                    .trim_start_matches("{")
                    .trim_end_matches("}")
                    .split(",")
                    .map(|t| {
                        let (k, v) = t
                            .split_once("=")
                            .ok_or_else(|| anyhow!("Bad component: '{}'", t))?;
                        let k = k.to_string();
                        let v: isize = v.parse().context("Parse component value")?;

                        Ok((k, v))
                    })
                    .collect::<Result<HashMap<String, isize>>>()?;

                parts.push(part);
            } else {
                // parse workflow
                let (name, rest) = line
                    .split_once("{")
                    .ok_or_else(|| anyhow!("Bad workflow line: '{}'", line))?;

                let name = name.to_string();

                let mut rules: Vec<&str> = rest.trim_end_matches("}").split(",").collect();
                let final_destination = rules
                    .pop()
                    .ok_or_else(|| anyhow!("Expected final destination"))?
                    .to_string();

                let rules: Vec<(Rule, String)> = rules
                    .into_iter()
                    .map(|r| Rule::parse(r))
                    .collect::<Result<Vec<(Rule, String)>>>()?;

                workflows.insert(
                    name,
                    Workflow {
                        rules,
                        final_destination,
                    },
                );
            }
        }

        Ok(Self { workflows, parts })
    }

    fn sort(&self) -> Result<isize> {
        let mut queue: VecDeque<(&str, HashMap<String, isize>)> = VecDeque::new();

        for part in &self.parts {
            queue.push_back(("in", part.clone()));
        }

        let mut accepted_sum = 0;

        while let Some((wf, part)) = queue.pop_front() {
            if wf == "A" {
                accepted_sum += part.values().sum::<isize>();
            } else if wf == "R" {
                // no need to continue with reject conditions
            } else {
                let workflow = self
                    .workflows
                    .get(wf)
                    .ok_or_else(|| anyhow!("Unknown workflow '{}'", wf))?;

                let dest = workflow.apply_to(&part);
                queue.push_back((dest, part));
            }
        }

        Ok(accepted_sum)
    }

    fn sort_symbolically(&self) -> Result<Vec<Hypercube>> {
        let mut out = Vec::new();
        let mut queue: VecDeque<(String, Vec<Hypercube>)> = VecDeque::new();

        queue.push_back(("in".to_string(), vec![Hypercube::new()]));

        while let Some((workflow, in_cubes)) = queue.pop_front() {
            if workflow == "A" {
                out.extend(in_cubes);
            } else if workflow == "R" {
                // no need to continue with reject conditions
            } else {
                let workflow = self
                    .workflows
                    .get(&workflow)
                    .ok_or_else(|| anyhow!("Unknown workflow '{}'", workflow))?;

                let mut out_cubes: HashMap<String, Vec<Hypercube>> = HashMap::new();
                for cube in in_cubes {
                    for (dest, out_cube) in workflow.apply_symbolically(&cube) {
                        out_cubes.entry(dest).or_default().push(out_cube);
                    }
                }

                for (destination, cubes) in out_cubes {
                    queue.push_back((destination, cubes));
                }
            }
        }

        Ok(out)
    }
}

/// An n-dimensional hypercube, using strings as dimension indices.
///
/// per-dimension extents are specified as (min, max) where min is inclusive and max exclusive.
#[derive(Debug, Clone)]
struct Hypercube {
    dimensions: HashMap<String, (isize, isize)>,
}

impl Hypercube {
    fn new() -> Self {
        let mut dimensions = HashMap::new();

        for dim in ["x", "m", "a", "s"] {
            dimensions.insert(dim.to_string(), (1, 4001));
        }

        Self { dimensions }
    }

    fn is_empty(&self) -> bool {
        self.dimensions.values().any(|(min, max)| min >= max)
    }

    fn volume(&self) -> isize {
        self.dimensions
            .values()
            .map(|(min, max)| if max > min { max - min } else { 0 })
            .product()
    }
}

impl std::ops::BitAnd for &Hypercube {
    type Output = Hypercube;

    fn bitand(self, rhs: Self) -> Self::Output {
        let dimensions = self
            .dimensions
            .iter()
            .map(|(key, (lmin, lmax))| {
                let (rmin, rmax) = rhs
                    .dimensions
                    .get(key)
                    .cloned()
                    .unwrap_or((isize::MIN, isize::MAX));

                let min = isize::max(*lmin, rmin);
                let max = isize::min(*lmax, rmax);

                (key.clone(), (min, max))
            })
            .collect();

        Hypercube { dimensions }
    }
}

fn sum_volumes(cubes: &[Hypercube]) -> isize {
    cubes.iter().map(|c| c.volume()).sum()
}

fn main() -> Result<()> {
    let input = Input::parse("data/day19/input")?;

    let part1 = input.sort()?;
    println!("Part 1: {}", part1);

    let part2 = input.sort_symbolically()?;
    println!("Part 2: {}", sum_volumes(&part2[..]));

    Ok(())
}
