use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    attributes: Vec<i64>,
    calories: i64,
}

const EXPECTED_ATTRIBUTES: [&str; 4] = ["capacity", "durability", "flavor", "texture"];

impl std::str::FromStr for Ingredient {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
        let mut tokens: Vec<_> = s.split(" ").collect();

        let name = tokens.remove(0).replace(":", "").to_owned();
        let calories: i64 = tokens.pop().unwrap().parse()?;
        tokens.pop(); // calories text

        let mut attributes = Vec::new();
        while !tokens.is_empty() {
            let label = tokens.remove(0);

            if label != EXPECTED_ATTRIBUTES[attributes.len()] {
                return Err(format_err!("Unexpected label '{}'", label));
            }

            let val: i64 = tokens.remove(0).replace(",", "").parse()?;
            attributes.push(val);
        }

        Ok(Ingredient {
            name,
            attributes,
            calories,
        })
    }
}

fn score(recipe: &HashMap<&Ingredient, i64>) -> (i64, i64) {
    let n_attributes = recipe.keys().next().unwrap().attributes.len();

    let mut attributes: Vec<i64> = (0..n_attributes).map(|_v| 0).collect();
    let mut calories = 0;

    for (ingredient, quantity) in recipe.iter() {
        for (i, v) in ingredient.attributes.iter().enumerate() {
            attributes[i] += quantity * v;
        }

        calories += quantity * ingredient.calories;
    }

    (
        attributes.iter().map(|a| std::cmp::max(*a, 0)).product(),
        calories,
    )
}

fn main() -> Result<()> {
    let mut ingredients: Vec<Ingredient> =
        read_to_parsed_lines("data/day15/input", &|l| l.parse())?;
    let a = ingredients.pop().unwrap();
    let b = ingredients.pop().unwrap();
    let c = ingredients.pop().unwrap();
    let d = ingredients.pop().unwrap();

    let mut recipe = HashMap::new();
    let mut top_recipe = HashMap::new();
    let mut top_score = 0;

    let mut c500_recipe = HashMap::new();
    let mut c500_score = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - (i + j)) {
                for l in 0..=(100 - (i + j + k)) {
                    recipe.insert(&a, i);
                    recipe.insert(&b, j);
                    recipe.insert(&c, k);
                    recipe.insert(&d, l);

                    let (cur_score, cur_calories) = score(&recipe);
                    if cur_score > top_score {
                        println!("New top recipe with score {}: {:?}", cur_score, recipe);
                        top_score = cur_score;
                        top_recipe = recipe.clone();
                    }

                    if cur_calories == 500 && cur_score > c500_score {
                        println!(
                            "New top 500kcal recipe with score {}: {:?}",
                            cur_score, recipe
                        );
                        c500_score = cur_score;
                        c500_recipe = recipe.clone();
                    }
                }
            }
        }
    }

    println!(
        "Part one: Best unconstrained   recipe with score {}: {:?}",
        top_score, top_recipe
    );
    println!(
        "Part two: Best 500-kilocalorie recipe with score {}: {:?}",
        c500_score, c500_recipe
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recipe_scoring() -> Result<()> {
        let butterscotch =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".parse()?;
        let cinnamon =
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".parse()?;

        let mut recipe = HashMap::new();
        recipe.insert(&butterscotch, 44);
        recipe.insert(&cinnamon, 56);

        assert_eq!(score(&recipe), (62842880, 520));

        Ok(())
    }
}
