use advent_of_code2020_rust::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day1(c: &mut Criterion) {
    let input = input_utils::read_all_as::<u32>("inputs/day1");
    c.bench_function("day 1 pt 1", |b| b.iter(|| day1::part1(&input)));
    c.bench_function("day 1 pt 2", |b| b.iter(|| day1::part2(&input)));
}

pub fn bench_day2(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day2");
    c.bench_function("day 2 pt 1", |b| {
        b.iter(|| day2::count_valid_passwords(&input))
    });
    c.bench_function("day 2 pt 2", |b| {
        b.iter(|| day2::count_valid_passwords_new_rules(&input))
    });
}

pub fn bench_day3(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day3");
    c.bench_function("day 3 pt 1", |b| {
        b.iter(|| day3::calculate_cut_trees_for_cheap_toboggan(&input))
    });
    c.bench_function("day 3 pt 2", |b| {
        b.iter(|| day3::calculate_slopes_cost_multiplied(&input))
    });
}

pub fn bench_day4(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day4");
    c.bench_function("day 4 pt 1", |b| {
        b.iter(|| day4::count_passwords_with_all_fields(&input))
    });
    c.bench_function("day 4 pt 2", |b| {
        b.iter(|| day4::count_valid_passwords(&input))
    });
}

pub fn bench_day5(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day5");
    c.bench_function("day 5 pt 1", |b| b.iter(|| day5::find_highest_id(&input)));
    c.bench_function("day 5 pt 2", |b| b.iter(|| day5::find_seat_id(&input)));
}

pub fn bench_day6(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day6");
    c.bench_function("day 6 pt 1", |b| {
        b.iter(|| day6::count_number_of_unique_group_yes_answers(&input))
    });
    c.bench_function("day 6 pt 2", |b| {
        b.iter(|| day6::count_number_of_unanimous_group_yes_answers(&input))
    });
}

pub fn bench_day7(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day7");
    c.bench_function("day 7 pt 1", |b| {
        b.iter(|| day7::count_bags_containing_shiny_gold(&input))
    });
    c.bench_function("day 7 pt 2", |b| {
        b.iter(|| day7::count_bags_inside_shiny_gold(&input))
    });
}

pub fn bench_day8(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day8");
    c.bench_function("day 8 pt 2", |b| {
        b.iter(|| day8::accumulator_value_fixing_loop(black_box(&input)))
    });

    let mut pt2_group = c.benchmark_group("Day 8 Pt 2");
    pt2_group.bench_function("day 8 pt 2", |b| {
        b.iter(|| day8::accumulator_value_fixing_loop(black_box(&input)))
    });
    pt2_group.bench_function("day 8 pt 2 - fast", |b| {
        b.iter(|| day8::accumulator_value_fixing_loop_fast(black_box(&input)))
    });
}

pub fn bench_day9(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day9");
    c.bench_function("day 9 pt 1", |b| {
        b.iter(|| day9::find_first_invalid(&input, 25))
    });
    c.bench_function("day 9 pt 2", |b| {
        b.iter(|| day9::find_vulnerability(&input, 25))
    });
}

pub fn bench_day10(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day10");
    c.bench_function("day 10 pt 1", |b| {
        b.iter(|| day10::find_chain_of_adapters_hash(&input))
    });
    c.bench_function("day 10 pt 2", |b| {
        b.iter(|| day10::count_total_combinations(&input))
    });
}

pub fn bench_day11(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day11");
    c.bench_function("day 11 pt 1", |b| b.iter(|| day11::part1(&input)));
    c.bench_function("day 11 pt 2", |b| b.iter(|| day11::part2(&input)));
}

pub fn bench_day12(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day12");
    c.bench_function("day 12 pt 1", |b| b.iter(|| day12::part1(&input)));
    c.bench_function("day 12 pt 2", |b| b.iter(|| day12::part2(&input)));
}

pub fn bench_day13(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day13");
    c.bench_function("day 13 pt 1", |b| b.iter(|| day13::part1(&input)));
    let mut pt2_group = c.benchmark_group("Day 13 Pt 2");
    pt2_group.bench_function("search using l.c.m.", |b| {
        b.iter(|| day13::part2_search(&input))
    });
    pt2_group.bench_function("chinese remainder", |b| {
        b.iter(|| day13::part2_chinese_remainder_theorem(&input))
    });
}

pub fn bench_day14(c: &mut Criterion) {
    let input = input_utils::read_all("inputs/day14");
    c.bench_function("day 14 pt 1", |b| b.iter(|| day14::part1(&input)));
    c.bench_function("day 14 pt 2", |b| b.iter(|| day14::part2(&input)));
}

pub fn bench_day15(c: &mut Criterion) {
    c.bench_function("day 15 pt 1", |b| b.iter(|| day15::part1()));
    c.bench_function("day 15 pt 2", |b| b.iter(|| day15::part2()));
}

criterion_group!(
    benches,
    bench_day1,
    bench_day2,
    bench_day3,
    bench_day4,
    bench_day5,
    bench_day6,
    bench_day7,
    bench_day8,
    bench_day9,
    bench_day10,
    bench_day11,
    bench_day12,
    bench_day13,
    bench_day14,
    bench_day15
);
criterion_main!(benches);
