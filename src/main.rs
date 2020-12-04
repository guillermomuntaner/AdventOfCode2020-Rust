mod input_utils;
mod day1;
mod day2;

fn main() {
    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");
    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));

    let day2_input = input_utils::read_all("inputs/day2");
    println!("Day 2 - Part 1: {}", day2::count_valid_passwords(&day2_input));
    println!("Day 2 - Part 1: {}", day2::count_valid_passwords_new_rules(&day2_input));
}
