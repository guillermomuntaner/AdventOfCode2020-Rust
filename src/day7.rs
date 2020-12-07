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

fn parse_bag_rule(line: &String) -> (String, Vec<String>) {
    lazy_static! {
        static ref OUTER_REGEX: Regex = Regex::new(r"([a-z]+ [a-z]+) bags contain").unwrap();
        static ref INNER_REGEX: Regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();
    }

    let outer: Captures = OUTER_REGEX.captures(line).unwrap();
    let outer_bag_color = outer.get(1).unwrap().as_str().to_string();
    //println!("{} contains:", outer_bag_color);

    let mut inner_bags: Vec<String> = Vec::new();
    for inner_cap in INNER_REGEX.captures_iter(line) {
        //let inner_bag_count = inner_cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let inner_bag_color = inner_cap.get(2).unwrap().as_str().to_string();
        //println!("- {} {} bag(s)", inner_bag_count, inner_bag_color);
        inner_bags.push(inner_bag_color);
    }

    return (outer_bag_color, inner_bags)
}

pub fn count_bags_containing_shiny_gold(lines: &Vec<String>) -> usize {
    // Dictionary of rules
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines.iter() {
        let rule =  parse_bag_rule(line);
        rules.insert(rule.0, rule.1);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_bag_rule() {
        assert_eq!(parse_bag_rule(&"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()),
                   ("light red".to_string() , vec!["bright white".to_string(), "muted yellow".to_string()]));
        assert_eq!(parse_bag_rule(&"dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string()),
                   ("dark orange".to_string() , vec!["bright white".to_string(), "muted yellow".to_string()]));
        assert_eq!(parse_bag_rule(&"bright white bags contain 1 shiny gold bag.".to_string()),
                   ("bright white".to_string() , vec!["shiny gold".to_string()]));
        assert_eq!(parse_bag_rule(&"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string()),
                   ("muted yellow".to_string() , vec!["shiny gold".to_string(), "faded blue".to_string()]));
        assert_eq!(parse_bag_rule(&"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string()),
                   ("shiny gold".to_string() , vec!["dark olive".to_string(), "vibrant plum".to_string()]));
        assert_eq!(parse_bag_rule(&"dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string()),
                   ("dark olive".to_string() , vec!["faded blue".to_string(), "dotted black".to_string()]));
        assert_eq!(parse_bag_rule(&"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string()),
                   ("vibrant plum".to_string() , vec!["faded blue".to_string(), "dotted black".to_string()]));
        assert_eq!(parse_bag_rule(&"faded blue bags contain no other bags.".to_string()),
                   ("faded blue".to_string() , vec![]));
        assert_eq!(parse_bag_rule(&"dotted black bags contain no other bags.".to_string()),
                   ("dotted black".to_string() , vec![]));
    }
}