use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use snafu::{ResultExt, Snafu};

use itertools::Itertools;

lazy_static! {
    static ref RE_FOOD: Regex = Regex::new(r"([a-z ]+) \(contains ([a-z, ]+)\)").unwrap();
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Food parsing error for '{}'", data))]
    ParseFood { data: String },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl std::str::FromStr for Food {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(caps) = RE_FOOD.captures(s) {
            let ingredients: HashSet<String> = caps
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            let allergens: HashSet<String> = caps
                .get(2)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect();

            Ok(Food {
                ingredients,
                allergens,
            })
        } else {
            Err(Error::ParseFood {
                data: s.to_string(),
            })
        }
    }
}

fn solve(foods: &[Food]) -> (HashMap<&str, &str>, HashSet<&str>) {
    // 1. Each allergen is found in exactly one ingredient.
    // 2. Each ingredient contains zero or one allergen.
    // 3. Allergens aren't always marked;
    // 4. When an allergen is listed (as in (contains nuts, shellfish) after an ingredients list),
    //    the ingredient that contains each listed allergen will be somewhere in the corresponding ingredients list.
    // 5. Even if an allergen isn't listed, the ingredient that contains that allergen could still be present

    // allergen -> which ingredients could have them
    let mut ai: HashMap<&str, HashSet<&str>> = HashMap::new();

    // list of all ingredients
    let mut ingredients: HashSet<&str> = HashSet::new();

    for f in foods {
        for i in &f.ingredients {
            ingredients.insert(i);
        }

        for a in &f.allergens {
            if ai.contains_key(&a[..]) {
                ai.get_mut(&a[..])
                    .unwrap()
                    .retain(|i| f.ingredients.contains(&i[..]));
            } else {
                ai.insert(a, f.ingredients.iter().map(|i| &i[..]).collect());
            }
        }
    }

    // potentially unsafe ingredients
    let pui: HashSet<&str> = ai.values().flat_map(|v| v.iter()).map(|i| *i).collect();
    let safe_ingredients: HashSet<&str> = ingredients.difference(&pui).map(|i| *i).collect();

    let mut making_progress = true;
    let mut allergen_to_ingredient: HashMap<&str, &str> = HashMap::new();

    while making_progress {
        making_progress = false;

        let mut remove: Vec<&str> = Vec::new();

        for (a, ingredients) in ai.iter() {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                allergen_to_ingredient.insert(a, ingredient);
                remove.push(*ingredient);
                making_progress = true;
            }
        }

        for i in remove {
            for ingredients in ai.values_mut() {
                ingredients.remove(i);
            }
        }
    }

    (allergen_to_ingredient, safe_ingredients)
}

fn main() -> Result<()> {
    let foods: Vec<Food> = std::fs::read_to_string("data/day21/input")
        .context(Io)?
        .lines()
        .map(|s| s.parse())
        .collect::<Result<_>>()?;

    let (allergen_ingredients, safe_ingredients) = solve(&foods);

    println!("{:?}\n{:?}", allergen_ingredients, safe_ingredients);

    let mut n_safe_ingredients = 0;
    for f in foods.iter() {
        for i in f.ingredients.iter() {
            if safe_ingredients.contains(&i[..]) {
                n_safe_ingredients += 1
            }
        }
    }

    println!("Part 1: got {} safe ingredients", n_safe_ingredients);

    let mut ai: Vec<(&str, &str)> = allergen_ingredients.iter().map(|(a, i)| (*a, *i)).collect();
    ai.sort();

    let canonical_ingredients = ai.into_iter().map(|(_, i)| i).join(",");
    println!("Part 2: {}", canonical_ingredients);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
