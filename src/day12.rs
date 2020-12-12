// --- Day 12: Rain Risk ---
// Your ferry made decent progress toward the island, but the storm came in faster than anyone
// expected. The ferry needs to take evasive actions!
//
// Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a
// route directly to safety, it produced extremely circuitous instructions. When the captain uses
// the PA system to ask if anyone can help, you quickly volunteer.
//
// The navigation instructions (your puzzle input) consists of a sequence of single-character
// actions paired with integer input values. After staring at them for a few minutes, you work out
// what they probably mean:
//
// Action N means to move north by the given value.
// Action S means to move south by the given value.
// Action E means to move east by the given value.
// Action W means to move west by the given value.
// Action L means to turn left the given number of degrees.
// Action R means to turn right the given number of degrees.
// Action F means to move forward by the given value in the direction the ship is currently facing.
// The ship starts by facing east. Only the L and R actions change the direction the ship is facing.
// (That is, if the ship is facing east and the next instruction is N10, the ship would move north
// 10 units, but would still move east if the following action were F.)
//
// For example:
//
// F10
// N3
// F7
// R90
// F11
// These instructions would be handled as follows:
//
// F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
// N3 would move the ship 3 units north to east 10, north 3.
// F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
// R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
// F11 would move the ship 11 units south to east 17, south 8.
// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of
// its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//
// Figure out where the navigation instructions lead. What is the Manhattan distance between that
// location and the ship's starting position?

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    A(i32),
    F(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();
    }
    let cap: Captures = REGEX
        .captures(line)
        .unwrap_or_else(|| panic!("Unexpected line: {}", line));
    let action = cap.get(1).unwrap().as_str();
    let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    match action {
        "N" => Instruction::N(value),
        "S" => Instruction::S(value),
        "E" => Instruction::E(value),
        "W" => Instruction::W(value),
        "L" => Instruction::A((-value).rem_euclid(360)),
        "R" => Instruction::A((value).rem_euclid(360)),
        "F" => Instruction::F(value),
        _ => panic!("Unexpected instruction: {}", line),
    }
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|line| parse_instruction(line)).collect()
}

pub fn part1(lines: &[String]) -> i32 {
    let instructions = parse_instructions(lines);

    // 0º = E; rotation clockwise
    let mut angle = 0;
    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        match instruction {
            Instruction::N(value) => y += value,
            Instruction::S(value) => y -= value,
            Instruction::E(value) => x += value,
            Instruction::W(value) => x -= value,
            Instruction::A(value) => angle = (angle + value).rem_euclid(360),
            Instruction::F(value) => match angle {
                270 => y += value,
                90 => y -= value,
                0 => x += value,
                180 => x -= value,
                _ => panic!("Unexpected angle {}", angle),
            },
        }
    }
    x.abs() + y.abs()
}

// --- Part Two ---
// Before you can give the destination to the captain, you realize that the actual action meanings
// were printed on the back of the instructions the whole time.
//
// Almost all of the actions indicate how to move a waypoint which is relative to the ship's
// position:
//
// Action N means to move the waypoint north by the given value.
// Action S means to move the waypoint south by the given value.
// Action E means to move the waypoint east by the given value.
// Action W means to move the waypoint west by the given value.
// Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number
// of degrees.
// Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
// Action F means to move forward to the waypoint a number of times equal to the given value.
// The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative
// to the ship; that is, if the ship moves, the waypoint moves with it.
//
// For example, using the same instructions as above:
//
// F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north),
// leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the
// ship.
// N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship
// remains at east 100, north 10.
// F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving
// the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
// R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10
// units south of the ship. The ship remains at east 170, north 38.
// F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south),
// leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the
// ship.
// After these operations, the ship's Manhattan distance from its starting position is
// 214 + 72 = 286.
//
// Figure out where the navigation instructions actually lead. What is the Manhattan distance
// between that location and the ship's starting position?

pub fn part2(lines: &[String]) -> i32 {
    let instructions = parse_instructions(lines);

    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for instruction in instructions {
        match instruction {
            Instruction::N(value) => waypoint_y += value,
            Instruction::S(value) => waypoint_y -= value,
            Instruction::E(value) => waypoint_x += value,
            Instruction::W(value) => waypoint_x -= value,
            Instruction::A(angle) => {
                match angle {
                    0 => {}
                    90 => {
                        // x -> -y, y -> x
                        let tmp_x = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -tmp_x;
                    }
                    180 => {
                        // x -> -x, y -> -y
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    }
                    270 => {
                        // x -> y, y -> -x
                        let tmp_x = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = tmp_x;
                    }
                    _ => panic!("Unexpected angle {}", angle),
                }
            }
            Instruction::F(value) => {
                x += value * waypoint_x;
                y += value * waypoint_y;
            }
        }
    }
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ship_operations() {
        let input_text = "F10\nN3\nF7\nR90\nF11";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&input), 25);
    }

    #[test]
    pub fn test_ship_operations_with_waypoint() {
        let input_text = "F10\nN3\nF7\nR90\nF11";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(&input), 286);
    }
}
