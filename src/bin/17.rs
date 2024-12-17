use advent_of_code::{parse_unsigned};

advent_of_code::solution!(17);

#[derive(Clone)]
struct Computer {
    register_a: u128,
    register_b: u128,
    register_c: u128,
    program: Vec<u8>,
}

impl Computer {
    fn parse_from_bytes(bytes: &[u8]) -> Self {
        let mut i = 12;
        let register_a = parse_unsigned(bytes, &mut i);
        i += 12;
        let register_b = parse_unsigned(bytes, &mut i);
        i += 12;
        let register_c = parse_unsigned(bytes, &mut i);

        let mut program: Vec<u8> = Vec::new();

        i += 10;
        while i < bytes.len() {
            program.push(bytes[i] - b'0');
            i += 2;
        }

        Computer {
            register_a,
            register_b,
            register_c,
            program,
        }
    }

    fn run(&self) -> Vec<u8> {
        let mut output = Vec::new();

        let mut register_a = self.register_a;
        let mut register_b = self.register_b;
        let mut register_c = self.register_c;

        let mut ip = 0;
        while ip < self.program.len() - 1 {
            let instruction = self.program[ip];
            let combo_operand = self.program[ip + 1];

            let combo_operand = match combo_operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                _ => unreachable!("invalid combo operand: {}", combo_operand),
            };

            match instruction {
                0 => {
                    // The *adv* instruction (opcode *0*) performs division. The numerator is the value
                    // in the A register. The denominator is found by raising 2 to the power of the
                    // instruction's combo operand. The result of the division operation is truncated to
                    // an integer and then written to the A register.
                    let numerator = register_a;
                    let denominator = 2_u128.pow(combo_operand as u32);
                    register_a = numerator / denominator;
                }
                1 => {
                    // The *bxl* instruction (opcode *1*) calculates the bitwise XOR of register B and
                    // the instruction's literal operand, then stores the result in register B.
                    register_b = register_b ^ combo_operand;
                }
                2 => {
                    // The *bst* instruction (opcode *2*) calculates the value of its combo operand
                    // modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the
                    // B register
                    register_b = combo_operand % 8;
                }
                3 => {
                    //The *jnz* instruction (opcode *3*) does nothing if the A register is 0. However,
                    // if the A register is not zero, it jumps by setting the instruction pointer to
                    // the value of its literal operand; if this instruction jumps, the instruction
                    // pointer is not increased by 2 after this instruction.
                    if register_a != 0 {
                        ip = combo_operand as usize;
                        continue;
                    }
                }
                4 => {
                    // The *bxc* instruction (opcode *4*) calculates the bitwise XOR of register B and
                    // register C, then stores the result in register B.
                    register_b = register_b ^ register_c;
                }
                5 => {
                    // The *out* instruction (opcode *5*) calculates the value of its combo operand
                    // modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                    output.push((combo_operand % 8) as u8);
                }
                6 => {
                    // The *bdv* instruction (opcode *6*) works exactly like the adv instruction except
                    // that the result is stored in the B register. (The numerator is still read from
                    // the A register.)

                    let numerator = register_a;
                    let denominator = 2_u128.pow(combo_operand as u32);
                    register_b = numerator / denominator;
                }
                7 => {
                    // The *cdv* instruction (opcode *7*) works exactly like the adv instruction except
                    // that the result is stored in the C register. (The numerator is still read from
                    // the A register.)
                    let numerator = register_a;
                    let denominator = 2_u128.pow(combo_operand as u32);
                    register_c = numerator / denominator;
                }
                _ => unreachable!("invalid instruction: {}", instruction),
            }

            ip += 2;
        }

        output
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let computer = Computer::parse_from_bytes(bytes);

    let output = computer
        .run()
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    Some(output.join(","))
}



pub fn part_two(input: &str) -> Option<u128> {
    let mut computer = Computer::parse_from_bytes(input.as_bytes());
    let program = computer.program.clone();
    let program_len = program.len();

    let mut current_possibilities = (0..8).collect::<Vec<u128>>();

    for _ in 1..program_len {
        let mut next_possibilities = Vec::new();

        // For each possible value of `p` from the previous exponent
        for &p in &current_possibilities {
            for q in 0..8 {
                if p == 0 {
                    continue; // Skip when p == 0
                }

                let ra = 8 * p + q;
                computer.register_a = ra;
                let out = computer.run();
                let l = out.len();

                if out == program[program_len - l..] {
                    next_possibilities.push(ra);
                }
                if out == program {
                    return Some(ra);
                }
            }
        }

        current_possibilities = next_possibilities; // Move to the next exponent
    }

    // If we get here, we didn't find an exact match, return the last possible value
    current_possibilities.last().copied()
}

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
