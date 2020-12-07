// --- Day 7: Handy Haversacks ---
// You land at the regional airport in time for your next flight. In fact, it looks like you'll even
// have time to grab some food: all flights are currently delayed due to issues in luggage
// processing.
//
// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags
// and their contents; bags must be color-coded and must contain specific quantities of other
// color-coded bags. Apparently, nobody responsible for these regulations considered how long they
// would take to enforce!
//
// For example, consider the following rules:
//
// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
// These rules specify the required contents for 9 bag types. In this example, every faded blue bag
// is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//
// You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many
// different bag colors would be valid for the outermost bag? (In other words: how many colors can,
// eventually, contain at least one shiny gold bag?)
//
// In the above rules, the following options would be available to you:
//
// A bright white bag, which can hold your shiny gold bag directly.
// A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
// A dark orange bag, which can hold bright white and muted yellow bags, either of which could then
// hold your shiny gold bag.
// A light red bag, which can hold bright white and muted yellow bags, either of which could then
// hold your shiny gold bag.
// So, in this example, the number of bag colors that can eventually contain at least one shiny gold
// bag is 4.
//
// How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is
// quite long; make sure you get all of it.)
use lazy_static::lazy_static;
use regex::{Regex, Captures};
use std::collections::HashMap;

fn parse_bag_rule(line: &String) -> (String, Vec<(String, usize)>) {
    lazy_static! {
        static ref OUTER_REGEX: Regex = Regex::new(r"([a-z]+ [a-z]+) bags contain").unwrap();
        static ref INNER_REGEX: Regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();
    }

    let outer: Captures = OUTER_REGEX.captures(line).unwrap();
    let outer_bag_color = outer.get(1).unwrap().as_str().to_string();
    //println!("{} contains:", outer_bag_color);

    let mut inner_bags: Vec<(String, usize)> = Vec::new();
    for inner_cap in INNER_REGEX.captures_iter(line) {
        let inner_bag_count = inner_cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let inner_bag_color = inner_cap.get(2).unwrap().as_str().to_string();
        //println!("- {} {} bag(s)", inner_bag_count, inner_bag_color);
        inner_bags.push((inner_bag_color, inner_bag_count));
    }

    return (outer_bag_color, inner_bags)
}

pub fn count_bags_containing_shiny_gold(lines: &Vec<String>) -> usize {
    // Dictionary of rules
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines.iter() {
        let rule =  parse_bag_rule(line);
        rules.insert(rule.0, rule.1.iter().map(|r| r.0.clone()).collect());
    }

    // Cache of already checked bags
    let mut contains = HashMap::<String, bool>::new();

    fn find_if_contains_shiny_gold(rules: &HashMap<String, Vec<String>>, contains: &mut HashMap<String, bool>, bag_color: &String) -> bool {
        match contains.get(bag_color) {
            Some(contains_bag_color) => {
                return *contains_bag_color;
            }
            None => {
                for inner_color in rules.get(bag_color).unwrap().iter() {
                    if inner_color == "shiny gold" {
                        contains.insert(bag_color.clone(), true);
                        return true
                    } else if find_if_contains_shiny_gold(rules, contains, inner_color) {
                        contains.insert(bag_color.clone(), true);
                        return true
                    }
                }
                contains.insert(bag_color.clone(), false);
                return false
            }
        }
    }

    // Iterate over all rule colors.
    let mut count = 0_usize;
    for (bag_color, _) in &rules {
        if find_if_contains_shiny_gold(&rules, &mut contains, bag_color) {
            count += 1
        }
    }

    return count
}

// --- Part Two ---
// It's getting pretty expensive to fly these days - not because of ticket prices, but because of
// the ridiculous number of bags you need to buy!
//
// Consider again your shiny gold bag and the rules from the above example:
//
// faded blue bags contain 0 other bags.
// dotted black bags contain 0 other bags.
// vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
// dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2
// vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//
// Of course, the actual rules have a small chance of going several levels deeper than this example;
// be sure to count all of the bags, even if the nesting becomes topologically impractical!
//
// Here's another example:
//
// shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.
// In this example, a single shiny gold bag must contain 126 other bags.
//
// How many individual bags are required inside your single shiny gold bag?

pub fn count_bags_inside_shiny_gold(lines: &Vec<String>) -> usize {
    // Dictionary of rules
    let mut rules: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for line in lines.iter() {
        let rule =  parse_bag_rule(line);
        rules.insert(rule.0, rule.1);
    }

    // Cache of already checked bags & their capacity
    let mut capacities_cache = HashMap::<String, usize>::new();

    fn find_capacity(rules: &HashMap<String, Vec<(String, usize)>>, capacities_cache: &mut HashMap<String, usize>, bag_color: &String) -> usize {
        match capacities_cache.get(bag_color) {
            Some(quantity) => {
                return *quantity;
            }
            None => {
                let mut count = 0_usize;
                for inner_color in rules.get(bag_color).unwrap().iter() {
                    let inner_count = find_capacity(rules, capacities_cache, &inner_color.0);
                    println!("Need {} x {} which each contain {}", inner_color.1, inner_color.0, inner_count);
                    // Add the (bag iself + the inner bags) * number of times
                    count += inner_color.1 * (1 + inner_count);
                }
                capacities_cache.insert(bag_color.clone(), count);
                return count
            }
        }
    }

    return find_capacity(&rules, &mut capacities_cache, &"shiny gold".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_bag_rule() {
        assert_eq!(parse_bag_rule(&"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()),
                   ("light red".to_string() , vec![("bright white".to_string(), 1), ("muted yellow".to_string(), 2)]));
        assert_eq!(parse_bag_rule(&"dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string()),
                   ("dark orange".to_string() , vec![("bright white".to_string(), 3), ("muted yellow".to_string(), 4)]));
        assert_eq!(parse_bag_rule(&"bright white bags contain 1 shiny gold bag.".to_string()),
                   ("bright white".to_string() , vec![("shiny gold".to_string(), 1)]));
        assert_eq!(parse_bag_rule(&"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string()),
                   ("muted yellow".to_string() , vec![("shiny gold".to_string(), 2), ("faded blue".to_string(), 9)]));
        assert_eq!(parse_bag_rule(&"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string()),
                   ("shiny gold".to_string() , vec![("dark olive".to_string(), 1), ("vibrant plum".to_string(), 2)]));
        assert_eq!(parse_bag_rule(&"dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string()),
                   ("dark olive".to_string() , vec![("faded blue".to_string(), 3), ("dotted black".to_string(), 4)]));
        assert_eq!(parse_bag_rule(&"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string()),
                   ("vibrant plum".to_string() , vec![("faded blue".to_string(), 5), ("dotted black".to_string(), 6)]));
        assert_eq!(parse_bag_rule(&"faded blue bags contain no other bags.".to_string()),
                   ("faded blue".to_string() , vec![]));
        assert_eq!(parse_bag_rule(&"dotted black bags contain no other bags.".to_string()),
                   ("dotted black".to_string() , vec![]));
    }
}