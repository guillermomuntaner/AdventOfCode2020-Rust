// https://adventofcode.com/2020/day/2
//
// --- Day 2: Password Philosophy ---
// Your flight departs in a few days from the coastal airport; the easiest way down to the coast
// from here is via toboggan.
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong
// with our computers; we can't log in!" You ask if you can take a look.
//
// Their password database seems to be a little corrupted: some of the passwords wouldn't have been
// allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of passwords (according
// to the corrupted database) and the corporate policy when that password was set.
//
// For example, suppose you have the following list:
//
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
// Each line gives the password policy and then the password. The password policy indicates the
// lowest and highest number of times a given letter must appear for the password to be valid. For
// example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
// instances of b, but needs at least 1. The first and third passwords are valid: they contain one
// a or nine c, both within the limits of their respective policies.
//
// How many passwords are valid according to their policies?
use lazy_static::lazy_static;
use regex::{Captures, Regex};

// TODO: Read memory model
fn parse_line(line: &str) -> (i32, i32, char, &str) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }

    let cap: Captures = REGEX.captures(line).unwrap();

    let min = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let max = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let char = cap.get(3).unwrap().as_str().parse::<char>().unwrap();
    let password = cap.get(4).unwrap().as_str();

    (min, max, char, &password)
}

#[test]
pub fn test_parse_line() {
    let (min, max, char, password) = parse_line(&"1-3 a: abcde");
    assert_eq!(min, 1);
    assert_eq!(max, 3);
    assert_eq!(char, 'a');
    assert_eq!(password, "abcde");
}

fn validate(data: &(i32, i32, char, &str)) -> bool {
    let (min, max, char, password) = data;
    let occurrences = password.matches(*char).count() as i32;
    occurrences >= *min && occurrences <= *max
}

#[test]
pub fn test_validate() {
    // Valid examples:
    // 1-3 a: abcde
    // 2-9 c: ccccccccc
    assert_eq!(validate(&(1, 3, 'a', "abcde")), true);
    assert_eq!(validate(&(2, 9, 'c', "ccccccccc")), true);
    // Invalid example:
    // 1-3 b: cdefg
    assert_eq!(validate(&(1, 3, 'b', "cdefg")), false);
}

pub fn count_valid_passwords(lines: &[String]) -> usize {
    return lines
        .iter()
        .map(|line| parse_line(&line))
        .filter(|data| validate(data))
        .count();
}

// --- Part Two ---
// While it appears you validated the passwords correctly, they don't seem to be what the Official
// Toboggan Corporate Authentication System is expecting.
//
// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules
// from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy
// actually works a little differently.
//
// Each policy actually describes two positions in the password, where 1 means the first character,
// 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept
// of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences
// of the letter are irrelevant for the purposes of policy enforcement.
//
// Given the same example list from above:
//
// 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
// 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
// 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
// How many passwords are valid according to the new interpretation of the policies?

fn validate_new_rules(data: &(i32, i32, char, &str)) -> bool {
    let (min, max, char, password) = data;
    let first = password.chars().nth(*min as usize - 1).unwrap();
    let second = password.chars().nth(*max as usize - 1).unwrap();
    return vec![first, second].iter().filter(|&c| c == char).count() == 1;
}

#[test]
pub fn test_validate_new_rules() {
    // 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    assert_eq!(validate_new_rules(&(1, 3, 'a', "abcde")), true);
    // 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    assert_eq!(validate_new_rules(&(1, 3, 'b', "cdefg")), false);
    // 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
    assert_eq!(validate_new_rules(&(1, 3, 'a', "ccccccccc")), false);
}

pub fn count_valid_passwords_new_rules(lines: &[String]) -> usize {
    return lines
        .iter()
        .map(|line| parse_line(&line))
        .filter(|data| validate_new_rules(data))
        .count();
}
