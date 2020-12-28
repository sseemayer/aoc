use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_parsed_lines;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Reactant {
    count: i64,
    label: String,
}

impl std::fmt::Display for Reactant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.count, self.label)
    }
}

impl std::str::FromStr for Reactant {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens: Vec<_> = s.trim().split(" ").collect();

        let count: i64 = tokens.remove(0).parse()?;
        let label: String = tokens.remove(0).to_owned();

        Ok(Reactant { count, label })
    }
}

#[derive(Debug)]
struct Reaction {
    educts: Vec<Reactant>,
    product: Reactant,
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{} => {}",
            self.educts
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(" + "),
            self.product
        )
    }
}

impl std::str::FromStr for Reaction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rxn: Vec<_> = s.trim().split("=>").collect();

        let educts: Vec<Reactant> = rxn
            .remove(0)
            .split(",")
            .map(|t| t.parse())
            .collect::<Result<Vec<Reactant>>>()?;
        let product: Reactant = rxn.remove(0).parse()?;

        Ok(Reaction { educts, product })
    }
}

fn get_needed(need: &HashMap<String, i64>, supplied: &HashSet<String>) -> Option<(String, i64)> {
    for (label_need, count_need) in need {
        if *count_need > 0 && !supplied.contains(label_need) {
            return Some((label_need.to_owned(), *count_need));
        }
    }
    None
}

fn solve(
    rxns: &Vec<Reaction>,
    initial_need: Vec<Reactant>,
    supplied: &HashSet<String>,
) -> HashMap<String, i64> {
    let mut need = HashMap::new();

    for n in initial_need {
        need.insert(n.label, n.count);
    }

    loop {
        let (label_need, count_need) = get_needed(&need, supplied).unwrap();

        //println!("Attempting to solve for {}x{}", count_need, label_need);
        for rxn in rxns {
            if &label_need == &rxn.product.label {
                let count_product = rxn.product.count;

                let factor_product = (count_need + count_product - 1) / count_product;

                //println!(
                //    "Found rxn providing {}: {} factor={}",
                //    label_need, rxn, factor_product
                //);

                need.entry(label_need.clone())
                    .and_modify(|v| *v -= count_product * factor_product);

                for educt in rxn.educts.iter() {
                    need.entry(educt.label.to_owned())
                        .and_modify(|v| *v += educt.count * factor_product)
                        .or_insert(educt.count * factor_product);
                }
            }
        }

        //println!("Now need: {:?}\n", need);

        let mut only_supplied = true;
        for (n, c) in need.iter() {
            if *c > 0 && !supplied.contains(n) {
                only_supplied = false;
                break;
            }
        }

        if only_supplied {
            break;
        }
    }

    need
}

fn main() -> Result<()> {
    let rxns: Vec<Reaction> = read_to_parsed_lines("data/day14/input", &|l: &str| l.parse())?;
    println!("{:#?}", rxns);

    let mut supplied = HashSet::new();
    supplied.insert("ORE".to_owned());

    println!("FIRST RUN");
    let solution = solve(
        &rxns,
        vec![Reactant {
            label: "FUEL".to_owned(),
            count: 1,
        }],
        &supplied,
    );

    println!("Need ore: {}", solution["ORE"]);

    println!("SECOND RUN");

    let mut fuel_low = 0;
    let mut fuel_high = 1000000000;

    let target_ore = 1_000_000_000_000;

    loop {
        let fuel = (fuel_low + fuel_high) / 2;

        let ore_need = solve(
            &rxns,
            vec![Reactant {
                label: "FUEL".to_owned(),
                count: fuel,
            }],
            &supplied,
        )["ORE"];

        println!(
            "fuel {} < {} < {}: ore {}",
            fuel_low, fuel, fuel_high, ore_need
        );

        if ore_need < target_ore {
            fuel_low = fuel + 1;
        } else if ore_need > target_ore {
            fuel_high = fuel - 1;
        } else {
            println!("CAN MAKE {} FUEL", fuel);
        }

        if fuel_low > fuel_high {
            break;
        }
    }

    Ok(())
}
