// --- Day 8: Handheld Halting ---
// Your flight to the major airline hub reaches cruising altitude without incident. While you
// consider checking the in-flight menu for one of those drinks that come with a little umbrella,
// you are interrupted by the kid sitting next to you.
//
// Their handheld game console won't turn on! They ask if you can take a look.
//
// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of
// the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//
// The boot code is represented as a text file with one instruction per line of text. Each
// instruction consists of an operation (Acc, Jmp, or Nop) and an argument (a signed number like +4 or -20).
//
// Acc increases or decreases a single global value called the accumulator by the value given in
// the argument. For example, Acc +7 would increase the accumulator by 7. The accumulator starts at
// 0. After an Acc instruction, the instruction immediately below it is executed next.
// Jmp jumps to a new instruction relative to itself. The next instruction to execute is found using
// the argument as an offset from the Jmp instruction; for example, Jmp +2 would skip the next
// instruction, Jmp +1 would continue to the instruction immediately below it, and Jmp -20 would
// cause the instruction 20 lines above to be executed next.
// Nop stands for No OPeration - it does nothing. The instruction immediately below it is executed
// next.
// For example, consider the following program:
//
// Nop +0
// Acc +1
// Jmp +4
// Acc +3
// Jmp -3
// Acc -99
// Acc +1
// Jmp -4
// Acc +6
// These instructions are visited in this order:
//
// Nop +0  | 1
// Acc +1  | 2, 8(!)
// Jmp +4  | 3
// Acc +3  | 6
// Jmp -3  | 7
// Acc -99 |
// Acc +1  | 4
// Jmp -4  | 5
// Acc +6  |
// First, the Nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (Acc +1) and Jmp
// +4 sets the next instruction to the other Acc +1 near the bottom. After it increases the
// accumulator from 1 to 2, Jmp -4 executes, setting the next instruction to the only Acc +3. It
// sets the accumulator to 5, and Jmp -3 causes the program to continue back at the first Acc +1.
//
// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment
// the program tries to run any instruction a second time, you know it will never terminate.
//
// Immediately before the program would run an instruction a second time, the value in the
// accumulator is 5.
//
// Run your copy of the boot code. Immediately before any instruction is executed a second time,
// what value is in the accumulator?
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    //let REGEX = Regex::new(r"^(nop|acc|jmp) ([+\-]\d+)$").unwrap();
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(nop|acc|jmp) ([+\-]\d+)$").unwrap();
    }
    let cap: Captures = REGEX
        .captures(line)
        .unwrap_or_else(|| panic!("Unexpected line: {}", line));
    let operation = cap.get(1).unwrap().as_str();
    let argument = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    match operation {
        "nop" => Instruction::Nop(argument),
        "acc" => Instruction::Acc(argument),
        "jmp" => Instruction::Jmp(argument),
        _ => panic!("Unexpected instruction: {}", line),
    }
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|line| parse_instruction(line)).collect()
}

fn run_instruction(instruction: &Instruction, accumulator: &i32, position: &usize) -> (i32, usize) {
    match instruction {
        Instruction::Nop(_) => (*accumulator, position + 1),
        Instruction::Acc(acc) => (*accumulator + *acc, position + 1),
        Instruction::Jmp(pos) => (*accumulator, ((*position as i32) + pos) as usize),
    }
}

/// Executes the given instructions set. If it finds a loop it returns an error.
fn run_program(
    instructions: &[Instruction],
    accumulator: &i32,
    position: &usize,
    accessed_instructions: Option<&HashSet<usize>>,
) -> Result<i32, (i32, usize)> {
    let mut accumulator = *accumulator;
    let mut position = *position;
    let mut accessed_instructions = match accessed_instructions {
        Some(accessed_instructions) => accessed_instructions.clone(),
        None => HashSet::new(),
    };
    while position < instructions.len() {
        let (next_accumulator, next_position) =
            run_instruction(&instructions[position], &accumulator, &position);
        if accessed_instructions.contains(&next_position) {
            return Err((accumulator, position));
        }
        accessed_instructions.insert(position);
        accumulator = next_accumulator;
        position = next_position;
    }
    Ok(accumulator)
}

pub fn accumulator_value_before_entering_loop(lines: &[String]) -> i32 {
    let instructions = parse_instructions(lines);
    match run_program(&instructions, &0, &0, None) {
        Ok(_) => panic!("Expected an infinite loop"),
        Err(err) => err.0,
    }
}

// --- Part Two ---
// After some careful analysis, you believe that exactly one instruction is corrupted.
//
// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp.
// (No acc instructions were harmed in the corruption of this boot code.)
//
// The program is supposed to terminate by attempting to execute an instruction immediately after
// the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot
// code and make it terminate correctly.
//
// For example, consider the same program from above:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
// If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction
// infinite loop, never leaving that instruction. If you change almost any of the jmp instructions,
// the program will still eventually find another jmp instruction and loop forever.
//
// However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program
// terminates! The instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2
// jmp +4  | 3
// acc +3  |
// jmp -3  |
// acc -99 |
// acc +1  | 4
// nop -4  | 5
// acc +6  | 6
// After the last instruction (acc +6), the program terminates by attempting to run the instruction
// below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
//
// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to
// jmp). What is the value of the accumulator after the program terminates?

pub fn accumulator_value_fixing_loop(lines: &[String]) -> i32 {
    let instructions = parse_instructions(lines);
    for position in 0..instructions.len() {
        let mut modified_instructions = instructions.clone();
        match instructions[position] {
            Instruction::Nop(argument) => {
                modified_instructions[position] = Instruction::Jmp(argument)
            }
            Instruction::Acc(_) => continue,
            Instruction::Jmp(argument) => {
                modified_instructions[position] = Instruction::Nop(argument)
            }
        }
        if let Ok(acc) = run_program(&modified_instructions, &0, &0, None) {
            return acc;
        }
    }
    panic!("Didn't found any permutation that solves the loop");
}

pub fn accumulator_value_fixing_loop_fast(lines: &[String]) -> i32 {
    let instructions = parse_instructions(lines);
    let mut accumulator = 0;
    let mut position = 0;
    let mut accessed_instructions = HashSet::<usize>::new();
    while position < instructions.len() {
        // Check if the instruction can be swapped.
        let switched_op: Option<Instruction> = match instructions[position] {
            Instruction::Nop(argument) => Some(Instruction::Jmp(argument)),
            Instruction::Acc(_) => None,
            Instruction::Jmp(argument) => Some(Instruction::Nop(argument)),
        };
        // If there is the option to swap, run a single step with switched instruction,
        // then run the program from there.
        if let Some(instruction) = switched_op {
            let (next_accumulator, next_position) =
                run_instruction(&instruction, &accumulator, &position);
            if !accessed_instructions.contains(&next_position) {
                // TODO: Loop cache?
                if let Ok(acc) = run_program(
                    &instructions,
                    &next_accumulator,
                    &next_position,
                    Some(&accessed_instructions),
                ) {
                    return acc;
                }
            }
        }
        // Move 1 step further
        let (next_accumulator, next_position) =
            run_instruction(&instructions[position], &accumulator, &position);
        if accessed_instructions.contains(&next_position) {
            panic!("Both the permutation & the original instruction lead to loop");
        }
        accessed_instructions.insert(position);
        accumulator = next_accumulator;
        position = next_position;
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_instruction() {
        assert_eq!(
            parse_instruction(&String::from("nop +0")),
            Instruction::Nop(0)
        );
        assert_eq!(
            parse_instruction(&String::from("acc -117")),
            Instruction::Acc(-117)
        );
        assert_eq!(
            parse_instruction(&String::from("jmp +99")),
            Instruction::Jmp(99)
        );
    }
}
