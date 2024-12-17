use advent_of_code::parse_unsigned;

advent_of_code::solution!(17);

#[derive(Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    operations: [u8; 16],
}

impl Computer {
    fn parse_from_bytes(bytes: &[u8]) -> Self {
        let mut i = 12;
        let register_a = parse_unsigned(bytes, &mut i);
        i += 12;
        let register_b = parse_unsigned(bytes, &mut i);
        i += 12;
        let register_c = parse_unsigned(bytes, &mut i);
        i += 10;

        // let program: Vec<u8> = bytes[i..]
        //     .iter()
        //     .step_by(2)
        //     .map(|&b| b - b'0')
        //     .collect();

        let mut program = [0; 16];
        let mut program_len = 0;
        while i < bytes.len() {
            program[program_len] = bytes[i] - b'0';
            program_len += 1;
            i += 2;
        }

        Computer {
            register_a,
            register_b,
            register_c,
            operations: program,
        }
    }
    fn run(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(32);

        let mut register_a = self.register_a;
        let mut register_b = self.register_b;
        let mut register_c = self.register_c;

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
                0 => {
                    // The *adv* instruction (opcode *0*) performs division. The numerator is the value
                    // in the A register. The denominator is found by raising 2 to the power of the
                    // instruction's combo operand. The result of the division operation is truncated to
                    // an integer and then written to the A register.
                    register_a /= 1 << combo_operand;
                }
                1 => {
                    // The *bxl* instruction (opcode *1*) calculates the bitwise XOR of register B and
                    // the instruction's literal operand, then stores the result in register B.
                    register_b ^= combo_operand;
                }
                2 => {
                    // The *bst* instruction (opcode *2*) calculates the value of its combo operand
                    // modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the
                    // B register
                    register_b = combo_operand & 7;
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
                    register_b ^= register_c;
                }
                5 => {
                    // The *out* instruction (opcode *5*) calculates the value of its combo operand
                    // modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                    output.push(((combo_operand & 7) as u8) + b'0');
                    output.push(b',');
                }
                6 => {
                    // The *bdv* instruction (opcode *6*) works exactly like the adv instruction except
                    // that the result is stored in the B register. (The numerator is still read from
                    // the A register.)

                    register_b = register_a / (1 << combo_operand);
                }
                7 => {
                    // The *cdv* instruction (opcode *7*) works exactly like the adv instruction except
                    // that the result is stored in the C register. (The numerator is still read from
                    // the A register.)
                    register_c = register_a / (1 << combo_operand);
                }
                _ => unreachable!("invalid instruction: {}", instruction),
            }

            ip += 2;
        }

        output.pop();
        output
    }

    fn run_different_a(&self, a: u64) -> u32 {
        // let mut output = Vec::with_capacity(16);

        let mut output = 0;

        let mut register_a = a;
        let mut register_b = self.register_b;
        let mut register_c = self.register_c;

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
                },
                6 => register_b = register_a / (1 << combo_operand),
                7 => register_c = register_a / (1 << combo_operand),
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

    let program_output = computer.run();
    String::from_utf8(program_output).ok()
}

fn array_to_num(arr: &[u8]) -> u32 {
    arr.iter().fold(0, |acc, &n| acc * 10 + n as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = Computer::parse_from_bytes(input.as_bytes());
    let program_len = computer.operations.len();


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
                        let out = computer.run_different_a(ra);
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
