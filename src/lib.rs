pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod input_utils;

/// Test all previous results stay stable.
#[cfg(test)]
mod tests {
    use crate::{
        day1, day10, day11, day12, day13, day14, day15, day2, day3, day4, day5, day6, day7, day8, day9,
        input_utils,
    };

    #[test]
    pub fn test_day1() {
        let day1_input = input_utils::read_all_as::<u32>("inputs/day1");
        assert_eq!(day1::part1(&day1_input), 877971);
        assert_eq!(day1::part2(&day1_input), 203481432);
    }

    #[test]
    pub fn test_day2() {
        let day2_input = input_utils::read_all("inputs/day2");
        assert_eq!(day2::count_valid_passwords(&day2_input), 548);
        assert_eq!(day2::count_valid_passwords_new_rules(&day2_input), 502);
    }

    #[test]
    pub fn test_day3() {
        let day3_input = input_utils::read_all("inputs/day3");
        assert_eq!(
            day3::calculate_cut_trees_for_cheap_toboggan(&day3_input),
            259
        );
        assert_eq!(
            day3::calculate_slopes_cost_multiplied(&day3_input),
            2224913600
        );
    }

    #[test]
    pub fn test_day4() {
        let day4_input = input_utils::read_all("inputs/day4");
        assert_eq!(day4::count_passwords_with_all_fields(&day4_input), 235);
        assert_eq!(day4::count_valid_passwords(&day4_input), 194);
    }

    #[test]
    pub fn test_day5() {
        let day5_input = input_utils::read_all("inputs/day5");
        assert_eq!(day5::find_highest_id(&day5_input), 828);
        assert_eq!(day5::find_seat_id(&day5_input), 565);
    }

    #[test]
    pub fn test_day6() {
        let day6_input = input_utils::read_all("inputs/day6");
        assert_eq!(
            day6::count_number_of_unique_group_yes_answers(&day6_input),
            6878
        );
        assert_eq!(
            day6::count_number_of_unanimous_group_yes_answers(&day6_input),
            3464
        );
    }

    #[test]
    pub fn test_day7() {
        let day7_input = input_utils::read_all("inputs/day7");
        assert_eq!(day7::count_bags_containing_shiny_gold(&day7_input), 326);
        assert_eq!(day7::count_bags_inside_shiny_gold(&day7_input), 5635);
    }

    #[test]
    pub fn test_day8() {
        let day8_input = input_utils::read_all("inputs/day8");
        assert_eq!(
            day8::accumulator_value_before_entering_loop(&day8_input),
            1521
        );
        assert_eq!(day8::accumulator_value_fixing_loop(&day8_input), 1016);
        assert_eq!(day8::accumulator_value_fixing_loop_fast(&day8_input), 1016);
    }

    #[test]
    pub fn test_day9() {
        let day9_input = input_utils::read_all("inputs/day9");
        assert_eq!(day9::find_first_invalid(&day9_input, 25), 29221323);
        assert_eq!(day9::find_vulnerability(&day9_input, 25), 4389369);
    }

    #[test]
    pub fn test_day10() {
        let day10_input = input_utils::read_all("inputs/day10");
        assert_eq!(day10::find_chain_of_adapters_hash(&day10_input), 1904);
        assert_eq!(
            day10::count_total_combinations(&day10_input),
            10578455953408
        );
    }

    #[test]
    pub fn test_day11() {
        let day11_input = input_utils::read_all("inputs/day11");
        assert_eq!(day11::part1(&day11_input), 2249);
        assert_eq!(day11::part2(&day11_input), 2023);
    }

    #[test]
    pub fn test_day12() {
        let day12_input = input_utils::read_all("inputs/day12");
        assert_eq!(day12::part1(&day12_input), 1457);
        assert_eq!(day12::part2(&day12_input), 106860);
    }

    #[test]
    pub fn test_day13() {
        let day13_input = input_utils::read_all("inputs/day13");
        assert_eq!(day13::part1(&day13_input), 2095);
        assert_eq!(day13::part2_search(&day13_input), 598411311431841);
        assert_eq!(
            day13::part2_chinese_remainder_theorem(&day13_input),
            598411311431841
        );
    }

    #[test]
    pub fn test_day14() {
        let day14_input = input_utils::read_all("inputs/day14");
        assert_eq!(day14::part1(&day14_input), 2346881602152);
        assert_eq!(day14::part2(&day14_input), 3885232834169);
    }

    #[test]
    pub fn test_day15() {
        assert_eq!(day15::part1(), 475);
        assert_eq!(day15::part2(), 11261);
    }
}
