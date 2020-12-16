// --- Day 16: Ticket Translation ---
// As you're walking to yet another connecting flight, you realize that one of the legs of your
// re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in
// a language you don't understand. You should probably figure out what it says before you get to
// the train station after the next flight.
//
// Unfortunately, you can't actually read the words on the ticket. You can, however, read the
// numbers, and so you figure out the fields these tickets must have and the valid ranges for values
// in those fields.
//
// You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other
// nearby tickets for the same train service (via the airport security cameras) together into a
// single document you can reference (your puzzle input).
//
// The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the
// valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one
// of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7
// (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//
// Each ticket is represented by a single line of comma-separated values. The values are the numbers
// on the ticket in the order they appear; every ticket has the same format. For example, consider
// this ticket:
//
// .--------------------------------------------------------.
// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
// |                                                        |
// | ??: 301  ??: 302             ???????: 303      ??????? |
// | ??: 401  ??: 402           ???? ????: 403    ????????? |
// '--------------------------------------------------------'
// Here, ? represents text in a language you don't understand. This ticket might be represented as
// 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at
// are much more complicated. In any case, you've extracted just the numbers in such a way that the
// first number is always the same specific field, the second number is always a different specific
// field, and so on - you just don't know what each position actually means!
//
// Start by determining which tickets are completely invalid; these are tickets that contain values
// which aren't valid for any field. Ignore your ticket for now.
//
// For example, suppose you have the following notes:
//
// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
//
// your ticket:
// 7,1,14
//
// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12
// It doesn't matter which position corresponds to which field; you can identify invalid nearby
// tickets by considering only whether tickets contain values that are not valid for any field. In
// this example, the values on the first nearby ticket are all valid for at least one field. This is
// not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any
// field. Adding together all of the invalid values produces your ticket scanning error rate:
// 4 + 55 + 12 = 71.
//
// Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
use lazy_static::lazy_static;
use reduce::Reduce;
use regex::{Captures, Regex};
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug, Clone)]
struct Rule {
    field: String,
    ranges: Vec<RangeInclusive<usize>>,
}

fn parse_instruction(line: &str) -> Rule {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-z\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let cap: Captures = REGEX
        .captures(line)
        .unwrap_or_else(|| panic!("Unexpected line: {}", line));
    let field = cap.get(1).unwrap().as_str();
    let range_1_min = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let range_1_max = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let range_2_min = cap.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let range_2_max = cap.get(5).unwrap().as_str().parse::<usize>().unwrap();
    Rule {
        field: field.to_string(),
        ranges: vec![range_1_min..=range_1_max, range_2_min..=range_2_max],
    }
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(|char| char == ',')
        .map(|number_str| number_str.parse::<usize>().unwrap())
        .collect()
}

pub fn part1(lines: &[String]) -> usize {
    let input_parts: Vec<_> = lines.split(|line| line == "").collect();

    let rules: Vec<_> = input_parts[0]
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let nearby_tickets: Vec<Vec<usize>> = input_parts[2]
        .iter()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    let valid_ranges: Vec<_> = rules.iter().flat_map(|rule| &rule.ranges).collect();

    nearby_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|value| !valid_ranges.iter().any(|range| range.contains(value)))
        })
        .sum()
}

// --- Part Two ---
// Now that you've identified which tickets contain invalid values, discard those tickets entirely.
// Use the remaining valid tickets to determine which field is which.
//
// Using the valid ranges for each field, determine what order the fields appear on the tickets. The
// order is consistent between all tickets: if seat is the third field, it is the third field on
// every ticket, including your ticket.
//
// For example, suppose you have the following notes:
//
// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19
//
// your ticket:
// 11,12,13
//
// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9
// Based on the nearby tickets in the above example, the first position must be row, the second
// position must be class, and the third position must be seat; you can conclude that in your
// ticket, class is 12, row is 11, and seat is 13.
//
// Once you work out which field is which, look for the six fields on your ticket that start with
// the word departure. What do you get if you multiply those six values together?

pub fn part2(lines: &[String]) -> usize {
    let input_parts: Vec<_> = lines.split(|line| line == "").collect();

    let mut rules: Vec<_> = input_parts[0]
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let nearby_tickets: Vec<_> = input_parts[2]
        .iter()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    let valid_ranges: Vec<_> = rules.iter().flat_map(|rule| &rule.ranges).collect();

    let valid_nearby_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| valid_ranges.iter().any(|range| range.contains(value)))
        })
        .collect();

    let my_ticket: Vec<_> = parse_ticket(input_parts[1].get(1).unwrap());

    // Find the field order.
    // Note: Some field rules are valid for different positions, however there is only one solution
    // that satisfies all positions.
    let fields_len = valid_nearby_tickets.first().unwrap().len();
    let mut fields: Vec<Option<String>> = vec![None; fields_len];

    'search: loop {
        for pos in 0..valid_nearby_tickets.first().unwrap().len() {
            if fields.iter().filter(|field| field.is_none()).count() == 0 {
                break 'search;
            }

            if fields[pos].is_some() {
                continue;
            }

            let values_at_pos: Vec<_> = valid_nearby_tickets
                .iter()
                .map(|ticket| ticket[pos])
                .collect();

            let fitting_rules: Vec<_> = rules
                .iter()
                .enumerate()
                .filter(|(_, rule)| {
                    values_at_pos
                        .iter()
                        .all(|value| rule.ranges.iter().any(|range| range.contains(value)))
                })
                .map(|(pos, _)| pos)
                .collect();

            // Remove the rule, as it helps reducing search.
            if fitting_rules.len() == 1 {
                let rule = rules.remove(fitting_rules[0]).field.clone();
                fields[pos] = Option::from(rule);
            }
        }
    }

    let fields: Vec<_> = fields.iter().map(|opt| opt.as_ref().unwrap()).collect();
    let departure_fields: Vec<_> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.starts_with("departure"))
        .collect();
    assert_eq!(departure_fields.len(), 6);

    departure_fields
        .iter()
        .map(|(position, _)| my_ticket[*position])
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_instruction() {
        assert_eq!(
            parse_instruction(&"arrival platform: 35-368 or 389-972"),
            Rule {
                field: String::from("arrival platform"),
                ranges: vec![35..=368, 389..=972]
            }
        );
        assert_eq!(
            parse_instruction(&"row: 35-736 or 743-957"),
            Rule {
                field: String::from("row"),
                ranges: vec![35..=736, 743..=957]
            }
        );
    }

    #[test]
    pub fn test_parse_ticket() {
        assert_eq!(
            parse_ticket(
                &"418,710,489,833,397,567,488,620,158,218,199,857,271,566,911,790,152,489,746,421"
            ),
            vec![
                418, 710, 489, 833, 397, 567, 488, 620, 158, 218, 199, 857, 271, 566, 911, 790,
                152, 489, 746, 421
            ]
        )
    }
}
