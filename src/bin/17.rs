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

    fn run_num_out(&self, a: u64) -> u32 {
        let mut output = 0;

        let mut register_a = a;
        let mut register_b = 0;
        let mut register_c = 0;

        let mut ip = 0;
        while ip < self.operations.len() - 1 {
            let instruction = self.operations[ip];
            let combo_operand = self.operations[ip + 1];

            let combo_operand = match combo_operand {
                0..=3 => combo_operand as u64,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                _ => unreachable!("invalid combo operand: {}", combo_operand),
            };

            match instruction {
                0 => register_a /= 1 << combo_operand,
                1 => register_b ^= combo_operand,
                2 => register_b = combo_operand & 7,
                3 => {
                    if register_a != 0 {
                        ip = combo_operand as usize;
                        continue;
                    }
                }
                4 => register_b ^= register_c,
                5 => {
                    output = (output * 10) + (combo_operand & 7) as u32;
                }
                6 => register_b = register_a / (1 << combo_operand),
                7 => register_c = register_a / (1 << combo_operand),
                _ => unreachable!("invalid instruction: {}", instruction),
            }

            ip += 2;
        }

        output
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let a = parse_unsigned(bytes, &mut 12);
    Some(solve_specific_program(a))
}

fn array_to_num(arr: &[u8]) -> u64 {
    arr.iter().fold(0, |acc, &n| (acc * 10) + n as u64)
}


fn solve_specific_program(ras: u64) -> u64 {
    let mut output = 0;

    let mut ra = ras;
    let mut rb = ra & 7;


    loop {
        rb ^= 3;
        rb ^= ra / (1 << rb);
        rb ^= 3;
        ra >>= 3;

        output = (output * 10) + (rb & 7);
        if ra == 0 {
            return output;
        }
        rb = ra & 7;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = Computer::parse_from_bytes(input.as_bytes());
    let program_len = computer.program_length;


    let mut current_possibilities = (1..8).collect::<Vec<u64>>();

    for i in 1..program_len {
        let target = array_to_num(&computer.operations[program_len - (i + 1)..]);

        current_possibilities = current_possibilities
            .iter()
            .flat_map(|&p| {
                (0..8)
                    .filter_map(|q| {
                        if p == 0 {
                            return None; // Skip when p == 0
                        }

                        let ra = 8 * p + q;
                        let out = solve_specific_program(ra);
                        if out == target {
                            return Some(ra);
                        }

                        return None;
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    // If we get here, we didn't find an exact match, return the last possible value
    current_possibilities.last().copied()
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
