// --- Day 14: Docking Data ---
// As your ferry approaches the sea port, the captain asks for your help again. The computer system that runs this port isn't compatible with the docking program on the ferry, so the docking parameters aren't being correctly initialized in the docking program's memory.
//
// After a brief inspection, you discover that the sea port's computer system uses a strange bitmask system in its initialization program. Although you don't have the correct decoder chip handy, you can emulate it in software!
//
// The initialization program (your puzzle input) can either update the bitmask or write a value to memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring bitmasks for a moment, a line like mem[8] = 11 would write the value 11 to memory address 8.
//
// The bitmask is always given as a string of 36 bits, written with the most significant bit (representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied to values immediately before they are written to memory: a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.
//
// For example, consider the following program:
//
// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// mem[8] = 11
// mem[7] = 101
// mem[8] = 0
// This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite two bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is overwritten with 1.
//
// The program then attempts to write the value 11 to memory address 8. By expanding everything out to individual bits, the mask is applied as follows:
//
// value:  000000000000000000000000000000001011  (decimal 11)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001001001  (decimal 73)
// So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program tries to write 101 to address 7:
//
// value:  000000000000000000000000000001100101  (decimal 101)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001100101  (decimal 101)
// This time, the mask has no effect, as the bits it overwrote were already the values the mask tried to set. Finally, the program tries to write 0 to address 8:
//
// value:  000000000000000000000000000000000000  (decimal 0)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001000000  (decimal 64)
// 64 is written to address 8 instead, overwriting the value that was there previously.
//
// To initialize your ferry's docking program, you need the sum of all values left in memory after the initialization program completes. (The entire 36-bit address space begins initialized to the value 0 at every address.) In the above example, only two values in memory are not zero - 101 (at address 7) and 64 (at address 8) - producing a sum of 165.
//
// Execute the initialization program. What is the sum of all values left in memory after it completes?
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

fn parse_instruction(line: &str) -> Instruction {
    lazy_static! {
        static ref REGEX_MASK: Regex = Regex::new(r"^mask = ([X10]{36})$").unwrap();
        static ref REGEX_MEM: Regex = Regex::new(r"^mem\[(\d+)] = (\d+)$").unwrap();
    }
    if let Some(cap) = REGEX_MASK.captures(line) {
        let str = cap.get(1).unwrap().as_str().to_string();
        return Instruction::Mask(str);
    }
    if let Some(cap) = REGEX_MEM.captures(line) {
        let address = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
        return Instruction::Mem(address, value);
    }
    panic!("Unexpected instruction: {}", line);
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|line| parse_instruction(line)).collect()
}

fn op(add_mask: u64, mask: u64, value: u64) -> u64 {
    (value | add_mask) & mask
}

fn decompose_mask(mask_str: &str) -> (u64, u64) {
    let add_str = mask_str
        .chars()
        .map(|c| if c == '1' { '1' } else { '0' })
        .collect::<String>();
    let add = u64::from_str_radix(&add_str, 2).unwrap();
    let mask_str = mask_str
        .chars()
        .map(|c| if c == '0' { '0' } else { '1' })
        .collect::<String>();
    let mask = u64::from_str_radix(&mask_str, 2).unwrap();
    (add, mask)
}

pub fn part1(lines: &[String]) -> u64 {
    let instructions = parse_instructions(lines);

    let mut current_add = 0_u64;
    let mut current_mask = 0_u64;
    let mut memory = HashMap::<u64, u64>::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                let (add, mask) = decompose_mask(&mask_str);
                current_add = add;
                current_mask = mask;
            }
            Instruction::Mem(address, value) => {
                memory.insert(address, op(current_add, current_mask, value));
            }
        }
    }

    memory.values().sum()
}

// --- Part Two ---
// For some reason, the sea port's computer system still can't communicate with your ferry's docking
// program. It must be using version 2 of the decoder chip!
//
// A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a
// memory address decoder. Immediately before a value is written to memory, each bit in the bitmask
// modifies the corresponding bit of the destination memory address in the following way:
//
// If the bitmask bit is 0, the corresponding memory address bit is unchanged.
// If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
// If the bitmask bit is X, the corresponding memory address bit is floating.
// A floating bit is not connected to anything and instead fluctuates unpredictably. In practice,
// this means the floating bits will take on all possible values, potentially causing many memory
// addresses to be written all at once!
//
// For example, consider the following program:
//
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1
// When this program goes to write to memory address 42, it first applies the bitmask:
//
// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X
// After applying the mask, four bits are overwritten, three of which are different, and two of
// which are floating. Floating bits take on every possible combination of values; with two floating
// bits, four actual memory addresses are written:
//
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
// Next, the program is about to write to memory address 26 with a different bitmask:
//
// address: 000000000000000000000000000000011010  (decimal 26)
// mask:    00000000000000000000000000000000X0XX
// result:  00000000000000000000000000000001X0XX
// This results in an address with three floating bits, causing writes to eight memory addresses:
//
// 000000000000000000000000000000010000  (decimal 16)
// 000000000000000000000000000000010001  (decimal 17)
// 000000000000000000000000000000010010  (decimal 18)
// 000000000000000000000000000000010011  (decimal 19)
// 000000000000000000000000000000011000  (decimal 24)
// 000000000000000000000000000000011001  (decimal 25)
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// The entire 36-bit address space still begins initialized to the value 0 at every address, and you
// still need the sum of all values left in memory at the end of the program. In this example, the
// sum is 208.
//
// Execute the initialization program using an emulator for a version 2 decoder chip. What is the
// sum of all values left in memory after it completes?

fn decompose_mask_v2(mask_str: &str) -> (u64, u64, Vec<u64>) {
    let mut or_mask = 0_u64;
    let mut and_mask = 0_u64;
    let mut floating_masks: Vec<u64> = vec![0];
    for (pos, char) in mask_str.chars().rev().enumerate() {
        let sum = 1 << pos;
        match char {
            'X' => {
                for floating_mask in floating_masks.clone() {
                    floating_masks.push(floating_mask | sum)
                }
            }
            '1' => {
                or_mask |= sum;
                and_mask |= sum;
            }
            '0' => and_mask |= sum,
            _ => panic!("Unexpected char {}", char),
        }
    }
    (or_mask, and_mask, floating_masks)
}

pub fn part2(lines: &[String]) -> u64 {
    let instructions = parse_instructions(lines);

    let mut current_or_mask = 0_u64;
    let mut current_and_mask = 0_u64;
    let mut floating_masks: Vec<u64> = Vec::new();
    let mut memory = HashMap::<u64, u64>::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                let (or_mask, and_mask, floating) = decompose_mask_v2(&mask_str);
                current_or_mask = or_mask;
                current_and_mask = and_mask;
                floating_masks = floating;
            }
            Instruction::Mem(address, value) => {
                let base_masked_address = (address | current_or_mask) & current_and_mask;
                for permutation in floating_masks.iter() {
                    let masked_address = base_masked_address + permutation;
                    memory.insert(masked_address, value);
                }
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part_1() {
        let (add, mask) = match parse_instruction(&"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X") {
            Instruction::Mask(mask_str) => decompose_mask(&mask_str),
            _ => panic!("unexpected parse"),
        };
        assert_eq!(op(add, mask, 11), 73);
        assert_eq!(op(add, mask, 101), 101);
        assert_eq!(op(add, mask, 11), 73);
        assert_eq!(op(add, mask, 0), 64);
    }
}
