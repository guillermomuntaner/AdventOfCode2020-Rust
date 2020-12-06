mod input_utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");
    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));

    let day2_input = input_utils::read_all("inputs/day2");
    println!("Day 2 - Part 1: {}", day2::count_valid_passwords(&day2_input));
    println!("Day 2 - Part 2: {}", day2::count_valid_passwords_new_rules(&day2_input));

    let day3_input = input_utils::read_all("inputs/day3");
    println!("Day 3 - Part 1: {}", day3::calculate_cut_trees_for_cheap_toboggan(&day3_input));
    println!("Day 3 - Part 2: {}", day3::calculate_slopes_cost_multiplied(&day3_input));

    let day4_input = input_utils::read_all("inputs/day4");
    println!("Day 4 - Part 1: {}", day4::count_passwords_with_all_fields(&day4_input));
    println!("Day 4 - Part 2: {}", day4::count_valid_passwords(&day4_input));

    let day5_input = input_utils::read_all("inputs/day5");
    println!("Day 5 - Part 1: {}", day5::find_highest_id(&day5_input));
    println!("Day 5 - Part 2: {}", day5::find_seat_id(&day5_input));

    let day6_input = input_utils::read_all("inputs/day6");
    println!("Day 6 - Part 1: {}", day6::find_total_number_of_unique_yes(&day6_input));
    println!("Day 6 - Part 2: {}", day6::find_total_number_of_perfect_yes(&day6_input));
}
