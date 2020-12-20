// --- Day 19: Monster Messages ---
// You land in an airport surrounded by dense forest. As you walk to your high-speed train, the
// Elves at the Mythical Information Bureau contact you again. They think their satellite has
// collected an image of a sea monster! Unfortunately, the connection to the satellite is having
// problems, and many of the messages sent back from the satellite have been corrupted.
//
// They sent you a list of the rules valid messages should obey and a list of received messages
// they've collected so far (your puzzle input).
//
// The rules for valid messages (the top part of your puzzle input) are numbered and build upon
// each other. For example:
//
// 0: 1 2
// 1: "a"
// 2: 1 3 | 3 1
// 3: "b"
// Some rules, like 3: "b", simply match a single character (in this case, b).
//
// The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means
// that to match rule 0, the text being checked must match rule 1, and the text after the part that
// matched rule 1 must then match rule 2.
//
// Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at
// least one list of sub-rules must match. (The ones that match might be different each time the
// rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text
// being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.
//
// Fortunately, there are no loops in the rules, so the list of possible matches will be finite.
// Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0
// matches aab or aba.
//
// Here's a more interesting example:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
// Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the same
// (aa or bb), and rule 3 matches two letters that are different (ab or ba).
//
// Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of letters,
// one pair with matching letters and one pair with different letters. This leaves eight
// possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.
//
// Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule
// 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
//
// The received messages (the bottom part of your puzzle input) need to be checked against the rules
// so you can determine which are valid and which are corrupted. Including the rules and the
// messages together, this might look like:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// ababbb
// bababa
// abbbab
// aaabbb
// aaaabbb
// Your goal is to determine the number of messages that completely match rule 0. In the above
// example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer 2.
// The whole message must match all of rule 0; there can't be extra unmatched characters in the
// message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched
// b on the end.)
//
// How many messages completely match rule 0?

use reduce::Reduce;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Rule {
    End(char),
    Bridge(Vec<Vec<u32>>),
}

fn parse_instruction(line: &str) -> (u32, Rule) {
    let mut idx: u32 = 0;
    let mut acc = 0_u32;
    let mut groups = Vec::<Vec<u32>>::new();
    let mut acc_group = Vec::<u32>::new();

    let mut chars = line.chars();
    while let Some(char) = chars.next() {
        match char {
            '"' => return (idx, Rule::End(chars.next().unwrap())),
            ':' => {
                chars.next();
                idx = acc;
                acc = 0;
            }
            '|' => {
                chars.next();
                groups.push(acc_group.clone());
                acc_group.clear();
            }
            ' ' => {
                acc_group.push(acc);
                acc = 0;
            }
            char => {
                acc = acc * 10 + char.to_digit(10).unwrap();
            }
        }
    }
    acc_group.push(acc);
    groups.push(acc_group.clone());
    return (idx, Rule::Bridge(groups));
}

fn count_valid_naive(messages: &[String], rules: HashMap<u32, Rule>) -> usize {
    fn fill(idx: &u32, rules: &HashMap<u32, Rule>) -> Vec<String> {
        match rules.get(idx).unwrap() {
            Rule::End(char) => vec![char.to_string()],
            Rule::Bridge(or_groups) => or_groups
                .iter()
                .flat_map(|group| {
                    group
                        .iter()
                        .map(|pointer| fill(pointer, rules))
                        .reduce(|left, right| {
                            left.iter()
                                .flat_map(|a| {
                                    right
                                        .iter()
                                        .map(|b| {
                                            let owned_string: String = a.to_owned();
                                            let another_owned_string: String = b.to_owned();
                                            format!("{}{}", owned_string, another_owned_string)
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap()
                })
                .collect::<Vec<_>>(),
        }
    }

    let valid_messages = fill(&0, &rules);

    messages
        .iter()
        .filter(|message| valid_messages.contains(message))
        .count()
}

fn count_valid(messages: &[String], rules: HashMap<u32, Rule>) -> usize {
    /// Recursively searches if the given string is valid according to the given rule and returns
    /// all valid possibilities it finds.
    fn is_valid(message: &String, pos: usize, idx: &u32, rules: &HashMap<u32, Rule>) -> Vec<usize> {
        if pos >= message.chars().count() {
            return vec![];
        }
        match rules.get(idx).unwrap() {
            Rule::End(char) if message.chars().nth(pos).unwrap() == *char => vec![pos + 1],
            Rule::End(_) => vec![],
            Rule::Bridge(or_groups) => {
                or_groups
                    .iter()
                    .flat_map(|group| {
                        // Find all chains of valid cases
                        group.iter().fold(vec![pos], |valid_cases, rule_idx| {
                            valid_cases
                                .iter()
                                .flat_map(|initial_pos| {
                                    is_valid(message, *initial_pos, rule_idx, rules)
                                })
                                .collect()
                        })
                    })
                    .collect()
            }
        }
    }

    messages
        .iter()
        .filter(|message| {
            is_valid(message, 0, &0_u32, &rules)
                .iter()
                .any(|pos| *pos == message.len())
        })
        .count()
}

pub fn part1_naive(lines: &[String]) -> usize {
    let input_parts: Vec<_> = lines.split(|line| line == "").collect();

    let rules: HashMap<_, _> = input_parts[0]
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let messages = input_parts[1];

    count_valid_naive(&messages, rules)
}

pub fn part1(lines: &[String]) -> usize {
    let input_parts: Vec<_> = lines.split(|line| line == "").collect();

    let rules: HashMap<_, _> = input_parts[0]
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let messages = input_parts[1];

    count_valid(&messages, rules)
}

// --- Part Two ---
// As you look over the list of messages, you realize your matching rules aren't quite right. To fix
// them, completely replace rules 8: 42 and 11: 42 31 with the following:
//
// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
// This small change has a big impact: now, the rules do contain loops, and the list of messages
// they could hypothetically match is infinite. You'll need to determine how these changes affect
// which messages are valid.
//
// Fortunately, many of the rules are unaffected by this change; it might help to start by looking
// at which rules always match the same set of values and how those rules (especially rules 42 and
// 31) are used by the new versions of rules 8 and 11.
//
// (Remember, you only need to handle the rules you have; building a solution that could handle any
// hypothetical combination of rules would be significantly more difficult.)
//
// For example:
//
// 42: 9 14 | 10 1
// 9: 14 27 | 1 26
// 10: 23 14 | 28 1
// 1: "a"
// 11: 42 31
// 5: 1 14 | 15 1
// 19: 14 1 | 14 14
// 12: 24 14 | 19 1
// 16: 15 1 | 14 14
// 31: 14 17 | 1 13
// 6: 14 14 | 1 14
// 2: 1 24 | 14 4
// 0: 8 11
// 13: 14 3 | 1 12
// 15: 1 | 14
// 17: 14 2 | 1 7
// 23: 25 1 | 22 14
// 28: 16 1
// 4: 1 1
// 20: 14 14 | 1 15
// 3: 5 14 | 16 1
// 27: 1 6 | 14 18
// 14: "b"
// 21: 14 1 | 1 14
// 25: 1 1 | 1 14
// 22: 14 14
// 8: 42
// 26: 14 22 | 1 20
// 18: 15 15
// 7: 14 5 | 1 21
// 24: 14 1
//
// abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
// bbabbbbaabaabba
// babbbbaabbbbbabbbbbbaabaaabaaa
// aaabbbbbbaaaabaababaabababbabaaabbababababaaa
// bbbbbbbaaaabbbbaaabbabaaa
// bbbababbbbaaaaaaaabbababaaababaabab
// ababaaaaaabaaab
// ababaaaaabbbaba
// baabbaaaabbaaaababbaababb
// abbbbabbbbaaaababbbbbbaaaababb
// aaaaabbaabaaaaababaa
// aaaabbaaaabbaaa
// aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
// babaaabbbaaabaababbaabababaaab
// aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
// Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba,
// ababaaaaaabaaab, and ababaaaaabbbaba.
//
// However, after updating rules 8 and 11, a total of 12 messages match:
//
// bbabbbbaabaabba
// babbbbaabbbbbabbbbbbaabaaabaaa
// aaabbbbbbaaaabaababaabababbabaaabbababababaaa
// bbbbbbbaaaabbbbaaabbabaaa
// bbbababbbbaaaaaaaabbababaaababaabab
// ababaaaaaabaaab
// ababaaaaabbbaba
// baabbaaaabbaaaababbaababb
// abbbbabbbbaaaababbbbbbaaaababb
// aaaaabbaabaaaaababaa
// aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
// aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
// After updating rules 8 and 11, how many messages completely match rule 0?

pub fn part2(lines: &[String], fixing_rules: bool) -> usize {
    let input_parts: Vec<_> = lines.split(|line| line == "").collect();

    let mut rules: HashMap<_, _> = input_parts[0]
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let messages = input_parts[1];

    if fixing_rules {
        rules.insert(8, Rule::Bridge(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Rule::Bridge(vec![vec![42, 31], vec![42, 11, 31]]));
    }

    // 414 is too low

    count_valid(messages, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("0: 4 1 5"),
            (0, Rule::Bridge(vec![vec![4, 1, 5]]))
        );
        assert_eq!(
            parse_instruction("1: 2 3 | 3 2"),
            (1, Rule::Bridge(vec![vec![2, 3], vec![3, 2]]))
        );
        assert_eq!(
            parse_instruction("2: 4 4 | 5 5"),
            (2, Rule::Bridge(vec![vec![4, 4], vec![5, 5]]))
        );
        assert_eq!(
            parse_instruction("3: 4 5 | 5 4"),
            (3, Rule::Bridge(vec![vec![4, 5], vec![5, 4]]))
        );
        assert_eq!(parse_instruction("4: \"a\""), (4, Rule::End('a')));
        assert_eq!(parse_instruction("5: \"b\""), (5, Rule::End('b')));
    }

    #[test]
    pub fn test_part1() {
        let input_text = "0: 4 1 5\n\
        1: 2 3 | 3 2\n\
        2: 4 4 | 5 5\n\
        3: 4 5 | 5 4\n\
        4: \"a\"\n\
        5: \"b\"\n\
        \n\
        ababbb\n\
        bababa\n\
        abbbab\n\
        aaabbb\n\
        aaaabbb";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1_naive(&input), 2);
    }

    #[test]
    pub fn test_part2() {
        let input_text = "42: 9 14 | 10 1\n\
        9: 14 27 | 1 26\n\
        10: 23 14 | 28 1\n\
        1: \"a\"\n\
        11: 42 31\n\
        5: 1 14 | 15 1\n\
        19: 14 1 | 14 14\n\
        12: 24 14 | 19 1\n\
        16: 15 1 | 14 14\n\
        31: 14 17 | 1 13\n\
        6: 14 14 | 1 14\n\
        2: 1 24 | 14 4\n\
        0: 8 11\n\
        13: 14 3 | 1 12\n\
        15: 1 | 14\n\
        17: 14 2 | 1 7\n\
        23: 25 1 | 22 14\n\
        28: 16 1\n\
        4: 1 1\n\
        20: 14 14 | 1 15\n\
        3: 5 14 | 16 1\n\
        27: 1 6 | 14 18\n\
        14: \"b\"\n\
        21: 14 1 | 1 14\n\
        25: 1 1 | 1 14\n\
        22: 14 14\n\
        8: 42\n\
        26: 14 22 | 1 20\n\
        18: 15 15\n\
        7: 14 5 | 1 21\n\
        24: 14 1\n\
        \n\
        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
        bbabbbbaabaabba\n\
        babbbbaabbbbbabbbbbbaabaaabaaa\n\
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
        bbbbbbbaaaabbbbaaabbabaaa\n\
        bbbababbbbaaaaaaaabbababaaababaabab\n\
        ababaaaaaabaaab\n\
        ababaaaaabbbaba\n\
        baabbaaaabbaaaababbaababb\n\
        abbbbabbbbaaaababbbbbbaaaababb\n\
        aaaaabbaabaaaaababaa\n\
        aaaabbaaaabbaaa\n\
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
        babaaabbbaaabaababbaabababaaab\n\
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(&input, false), 3);
        assert_eq!(part2(&input, true), 12);
    }
}
