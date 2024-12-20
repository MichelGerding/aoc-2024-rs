#![feature(portable_simd)]

use std::ops::Index;
use std::simd::prelude::*;
use lazy_static::lazy_static;
use advent_of_code::parse_unsigned;

advent_of_code::solution!(17);



#[derive(Clone, Debug)]
struct Computer {
    register_a: u64,
    operations: [u8; 16],
    program_length: usize,
}

impl Computer {
    fn parse_from_bytes(bytes: &[u8]) -> Self {
        let mut i = 12;
        let register_a = parse_unsigned(bytes, &mut i);
        i += 38;

        let mut program = [0; 16];
        let mut program_len = 0;
        while i < bytes.len() && program_len < 16 {
            program[program_len] = bytes[i] - b'0';
            program_len += 1;
            i += 2;
        }

        Computer {
            register_a,
            operations: program,
            program_length: program_len,
        }
    }

    fn run(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(32);

        let mut register_a = self.register_a;
        let mut register_b = 0;
        let mut register_c = 0;

        let mut ip = 0;
        while ip < self.operations.len() - 1 {
            let instruction = self.operations[ip];
            let combo_operand = self.operations[ip + 1];

            let combo_reg = match combo_operand {
                0..=3 => combo_operand as u64,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                _ => unreachable!("invalid combo operand: {}", combo_operand),
            };

            match instruction {
                0 => register_a /= 1 << combo_reg,
                1 => register_b ^= combo_reg,
                2 => register_b = combo_reg & 7,
                3 => {
                    if register_a != 0 {
                        ip = combo_reg as usize;
                        continue;
                    }
                }
                4 => register_b ^= register_c,
                5 => {
                    output.push(((combo_reg & 7) as u8) + b'0');
                    output.push(b',');
                }
                6 => register_b = register_a / (1 << combo_reg),
                7 => register_c = register_a / (1 << combo_reg),
                _ => unreachable!("invalid instruction: {}", instruction),
            }

            ip += 2;
        }

        output.pop();
        output
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let computer = Computer::parse_from_bytes(bytes);
    let output = computer.run();
    String::from_utf8(output).ok()
}

fn array_to_num(arr: &[u8]) -> u64 {
    arr.iter().fold(0, |acc, &n| (acc * 10) + n as u64)
}


lazy_static! {
    pub static ref ONE: u64x8 = u64x8::splat(1);
    pub static ref THREE: u64x8 = u64x8::splat(3);
    pub static ref SEVEN: u64x8 = u64x8::splat(7);
    pub static ref TEN: u64x8 = u64x8::splat(10);
}


fn solve_specific_program(ras: u64x8) -> u64x8 {
    let mut output = u64x8::splat(0);

    let mut ra = ras;
    let mut rb = u64x8::splat(0);

    let one = *ONE;
    let three = *THREE;
    let seven = *SEVEN;
    let ten = *TEN;

    loop {
        rb = ra & seven;
        rb ^= three;
        rb ^= ra / (one << rb);
        rb ^= three;
        ra >>= three;

        output = (output * ten) + (rb & seven);
        if ra[0] == 0 {
            return output;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = Computer::parse_from_bytes(input.as_bytes());
    let program_len = computer.program_length;

    let offsets = u64x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);

    unsafe {
        // Start with initial possibilities as a vector.
        let mut current_possibilities = offsets.as_array().to_vec();
        let mut next_possibilities = Vec::with_capacity(64); // Preallocate enough space for next possibilities.

        for i in 1..program_len {
            let target = u64x8::splat(array_to_num(&computer.operations[program_len - (i + 1)..program_len]));
            next_possibilities.clear(); // Clear the next_possibilities vector before reuse.

            // SIMD processing for current possibilities.
            for &p in &current_possibilities {
                let ra = u64x8::splat(8 * p) + offsets;
                let out = solve_specific_program(ra);
                let mask = out.simd_eq(target);

                // Inline SIMD mask to extract valid indices directly.
                let valid = mask.to_bitmask(); // Bitmask for valid indices (1 bit per element).
                let mut j = valid;
                while j != 0 {
                    let idx = j.trailing_zeros() as usize;
                    next_possibilities.push(*ra.index(idx)); // Fast access without bounds check.
                    j &= j - 1; // Clear the least significant set bit.
                }
            }

            // Swap current and next buffers (avoid reallocating).
            std::mem::swap(&mut current_possibilities, &mut next_possibilities);
        }

        // Return the last valid possibility, if any.
        current_possibilities.last().copied()
    }
}

// 108107566389757
// 108107566389757
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("5,7,3,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
