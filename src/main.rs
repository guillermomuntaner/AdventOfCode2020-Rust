use advent_of_code2020_rust::*;

fn main() {
    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");
    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));

    let day2_input = input_utils::read_all("inputs/day2");
    println!(
        "Day 2 - Part 1: {}",
        day2::count_valid_passwords(&day2_input)
    );
    println!(
        "Day 2 - Part 2: {}",
        day2::count_valid_passwords_new_rules(&day2_input)
    );

    let day3_input = input_utils::read_all("inputs/day3");
    println!(
        "Day 3 - Part 1: {}",
        day3::calculate_cut_trees_for_cheap_toboggan(&day3_input)
    );
    println!(
        "Day 3 - Part 2: {}",
        day3::calculate_slopes_cost_multiplied(&day3_input)
    );

    let day4_input = input_utils::read_all("inputs/day4");
    println!(
        "Day 4 - Part 1: {}",
        day4::count_passwords_with_all_fields(&day4_input)
    );
    println!(
        "Day 4 - Part 2: {}",
        day4::count_valid_passwords(&day4_input)
    );

    let day5_input = input_utils::read_all("inputs/day5");
    println!("Day 5 - Part 1: {}", day5::find_highest_id(&day5_input));
    println!("Day 5 - Part 2: {}", day5::find_seat_id(&day5_input));

    let day6_input = input_utils::read_all("inputs/day6");
    println!(
        "Day 6 - Part 1: {}",
        day6::count_number_of_unique_group_yes_answers(&day6_input)
    );
    println!(
        "Day 6 - Part 2: {}",
        day6::count_number_of_unanimous_group_yes_answers(&day6_input)
    );

    let day7_input = input_utils::read_all("inputs/day7");
    println!(
        "Day 7 - Part 1: {}",
        day7::count_bags_containing_shiny_gold(&day7_input)
    );
    println!(
        "Day 7 - Part 2: {}",
        day7::count_bags_inside_shiny_gold(&day7_input)
    );

    let day8_input = input_utils::read_all("inputs/day8");
    println!(
        "Day 8 - Part 1: {}",
        day8::accumulator_value_before_entering_loop(&day8_input)
    );
    println!(
        "Day 8 - Part 2: {}",
        day8::accumulator_value_fixing_loop(&day8_input)
    );
    println!(
        "Day 8 - Part 2b: {}",
        day8::accumulator_value_fixing_loop_fast(&day8_input)
    );

    let day9_input = input_utils::read_all("inputs/day9");
    println!(
        "Day 9 - Part 1: {}",
        day9::find_first_invalid(&day9_input, 25)
    );
    println!(
        "Day 9 - Part 2: {}",
        day9::find_vulnerability(&day9_input, 25)
    );

    let day10_input = input_utils::read_all("inputs/day10");
    println!(
        "Day 10 - Part 1: {}",
        day10::find_chain_of_adapters_hash(&day10_input)
    );
    println!(
        "Day 10 - Part 2: {}",
        day10::count_total_combinations(&day10_input)
    );

    let day11_input = input_utils::read_all("inputs/day11");
    println!("Day 11 - Part 1: {}", day11::part1(&day11_input));
    println!("Day 11 - Part 2: {}", day11::part2(&day11_input));

    let day12_input = input_utils::read_all("inputs/day12");
    println!("Day 12 - Part 1: {}", day12::part1(&day12_input));
    println!("Day 12 - Part 2: {}", day12::part2(&day12_input));

    let day13_input = input_utils::read_all("inputs/day13");
    println!("Day 13 - Part 1: {}", day13::part1(&day13_input));
    println!(
        "Day 13 - Part 2: {}",
        day13::part2_chinese_remainder_theorem(&day13_input)
    );

    let day14_input = input_utils::read_all("inputs/day14");
    println!("Day 14 - Part 1: {}", day14::part1(&day14_input));
    println!("Day 14 - Part 2: {}", day14::part2(&day14_input));

    println!("Day 15 - Part 1: {}", day15::part1_preallocated_arrray());
    println!("Day 15 - Part 1: {}", day15::part2_preallocated_array());

    let day16_input = input_utils::read_all("inputs/day16");
    println!("Day 16 - Part 1: {}", day16::part1(&day16_input));
    println!("Day 16 - Part 2: {}", day16::part2(&day16_input));

    let day17_input = input_utils::read_all("inputs/day17");
    println!("Day 17 - Part 1: {}", day17::part1(&day17_input));
    println!("Day 17 - Part 2: {}", day17::part2(&day17_input));

    let day18_input = input_utils::read_all("inputs/day18");
    println!("Day 18 - Part 1: {}", day18::part1(&day18_input));
    println!("Day 18 - Part 2: {}", day18::part2(&day18_input));

    let day19_input = input_utils::read_all("inputs/day19");
    println!("Day 19 - Part 1: {}", day19::part1(&day19_input));
    println!("Day 19 - Part 2: {}", day19::part2(&day19_input, true));
}
