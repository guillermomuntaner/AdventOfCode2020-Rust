// --- Day 21: Allergen Assessment ---
// You reach the train's last stop and the closest you can get to your vacation island without
// getting wet. There aren't even any boats here, but nothing can stop you now: you build a raft.
// You just need a few days' worth of food for your journey.
//
// You don't speak the local language, so you can't read any ingredients lists. However, sometimes,
// allergens are listed in a language you do understand. You should be able to use this information
// to determine which ingredient contains which allergen and work out which foods are safe to take
// with you on your trip.
//
// You start by compiling a list of foods (your puzzle input), one food per line. Each line includes
// that food's ingredients list followed by some or all of the allergens the food contains.
//
// Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen.
// Allergens aren't always marked; when they're listed (as in (contains nuts, shellfish) after an
// ingredients list), the ingredient that contains each listed allergen will be somewhere in the
// corresponding ingredients list. However, even if an allergen isn't listed, the ingredient that
// contains that allergen could still be present: maybe they forgot to label it, or maybe it was
// labeled in a language you don't know.
//
// For example, consider the following list of foods:
//
// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)
// The first food in the list has four ingredients (written in a language you don't understand):
// mxmxvkd, kfcds, sqjhc, and nhms. While the food might contain other allergens, a few allergens
// the food definitely contains are listed afterward: dairy and fish.
//
// The first step is to determine which ingredients can't possibly contain any of the allergens in
// any food in your list. In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh
// can contain an allergen. Counting the number of times any of these ingredients appear in any
// ingredients list produces 5: they all appear once each except sbzzf, which appears twice.
//
// Determine which ingredients cannot possibly contain any of the allergens in your list.
// How many times do any of those ingredients appear?

use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

fn parse_food(line: &str) -> Food {
    let mut s = line.to_string();
    s.truncate(s.len() - 1);

    let mut pars = s.split(" (");
    let ingredients_str = pars.next().unwrap();
    let allergens_str: String = pars.next().unwrap().chars().skip(9).collect();

    Food {
        ingredients: ingredients_str.split(' ').map(|str| str.to_string()).collect(),
        allergens: allergens_str.split(", ").map(|str| str.to_string()).collect()
    }
}

pub fn part1(lines: &[String]) -> usize {

    let foods: Vec<_> = lines.iter().map(|line| parse_food(line)).collect();

    let mut allergens_to_possible_ingredients = HashMap::<String, HashSet<String>>::new();

    for food in &foods {
        for allergen in &food.allergens {
            let ingredients: HashSet<String> = food.ingredients.clone();
            match allergens_to_possible_ingredients.get(allergen) {
                None => {
                    allergens_to_possible_ingredients.insert(allergen.clone(), ingredients);
                },
                Some(possible_ingredients) => {
                    let intersection = possible_ingredients.intersection(&ingredients).cloned().collect::<HashSet<String>>();
                    allergens_to_possible_ingredients.insert(allergen.clone(), intersection);
                }
            }
        }
    }

    println!("{:?}", allergens_to_possible_ingredients);

    let ingredients_that_may_contain_allergens: HashSet<String> = allergens_to_possible_ingredients.values().flatten().cloned().collect();
    println!("Ingredients that may contain allergens: {:?}", ingredients_that_may_contain_allergens);

    foods.iter().map(|food| {
        food.ingredients.iter()
            .filter(|ingredient| !ingredients_that_may_contain_allergens.contains(ingredient.clone()))
            .count()
    }).sum()
}

// --- Part Two ---
// Now that you've isolated the inert ingredients, you should have enough information to figure out
// which ingredient contains which allergen.
//
// In the above example:
//
// mxmxvkd contains dairy.
// sqjhc contains fish.
// fvjkl contains soy.
// Arrange the ingredients alphabetically by their allergen and separate them by commas to produce
// your canonical dangerous ingredient list. (There should not be any spaces in your canonical
// dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.
//
// Time to stock your raft with supplies. What is your canonical dangerous ingredient list?

pub fn part2(lines: &[String]) -> String {
    let foods: Vec<_> = lines.iter().map(|line| parse_food(line)).collect();

    let mut allergens_to_possible_ingredients = HashMap::<String, HashSet<String>>::new();

    for food in &foods {
        for allergen in &food.allergens {
            let ingredients: HashSet<String> = food.ingredients.clone();
            match allergens_to_possible_ingredients.get(allergen) {
                None => {
                    allergens_to_possible_ingredients.insert(allergen.clone(), ingredients);
                },
                Some(possible_ingredients) => {
                    let intersection = possible_ingredients.intersection(&ingredients).cloned().collect::<HashSet<String>>();
                    allergens_to_possible_ingredients.insert(allergen.clone(), intersection);
                }
            }
        }
    }

    let mut allergen_ingredient: Vec<(String, String)> = Vec::new();

    while !allergens_to_possible_ingredients.is_empty() {
        let copy = allergens_to_possible_ingredients.clone();
        for (allergen, ingredients) in copy {
            let unidentified_ingredients: Vec<_> = ingredients.iter()
                .filter(|ingredient| allergen_ingredient.iter().find(|(_, identified_ingredient)| *ingredient == identified_ingredient).is_none())
                .collect();
            if unidentified_ingredients.len() == 1 {
                allergen_ingredient.push((allergen.clone(), unidentified_ingredients[0].clone()));
                allergens_to_possible_ingredients.remove(&allergen);
            }
        }

    }

    println!("{:?}", allergen_ingredient);


    allergen_ingredient.sort_by_key(|(allergen,_)| allergen.clone());
    allergen_ingredient.iter().map(|(_, ingredient)| ingredient).cloned().collect::<Vec<String>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_instruction() {
        assert_eq!(
            parse_food(&"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            Food {
                ingredients: vec!["mxmxvkd".to_string(), "kfcds".to_string(), "sqjhc".to_string(), "nhms".to_string()].iter().cloned().collect(),
                allergens: vec!["dairy".to_string(), "fish".to_string()].iter().cloned().collect()
            }
        )
    }

    #[test]
    pub fn test_part_1() {
        let input_text = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
        trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
        sqjhc fvjkl (contains soy)\n\
        sqjhc mxmxvkd sbzzf (contains fish)";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    pub fn test_part_2() {
        let input_text = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
        trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
        sqjhc fvjkl (contains soy)\n\
        sqjhc mxmxvkd sbzzf (contains fish)";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(&input), String::from("mxmxvkd,sqjhc,fvjkl"));
    }
}