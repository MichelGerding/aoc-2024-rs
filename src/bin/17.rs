use std::ops::Div;
use advent_of_code::parse_u32;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let bytes = input.as_bytes();


    // parse the registers
    let mut i = 12;
    let mut register_a = parse_u32(bytes, &mut i); i += 12;
    let mut register_b = parse_u32(bytes, &mut i); i += 12;
    let mut registers_c = parse_u32(bytes, &mut i);

    let mut program: Vec<u8> = Vec::new();

    i += 10;
    while i < bytes.len() {
        program.push(bytes[i] - b'0');
        i+= 2;
    }

    let mut output = Vec::new();

    let mut ip = 0;
    while ip < program.len() - 1 {
        let instruction = program[ip];
        let combo_operand = program[ip + 1];

        let combo_operand = match combo_operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register_a,
            5 => register_b,
            6 => registers_c,
            _ => unreachable!("invalid combo operand: {}", combo_operand),
        };

        match instruction {
            0 => {
                // The *adv* instruction (opcode *0*) performs division. The numerator is the value
                // in the A register. The denominator is found by raising 2 to the power of the
                // instruction's combo operand. The result of the division operation is truncated to
                // an integer and then written to the A register.
                let numerator = register_a;
                let denominator = 2_u32.pow(combo_operand);
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
                    continue
                }
            }
            4 => {
                // The *bxc* instruction (opcode *4*) calculates the bitwise XOR of register B and
                // register C, then stores the result in register B.
                register_b = register_b ^ registers_c;
            }
            5 => {
                // The *out* instruction (opcode *5*) calculates the value of its combo operand
                // modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                output.push(format!("{}", combo_operand % 8));
            }
            6 => {
                // The *bdv* instruction (opcode *6*) works exactly like the adv instruction except
                // that the result is stored in the B register. (The numerator is still read from
                // the A register.)

                let numerator = register_a;
                let denominator = 2_u32.pow(combo_operand);
                register_b = numerator / denominator;
            }
            7 => {
                // The *cdv* instruction (opcode *7*) works exactly like the adv instruction except
                // that the result is stored in the C register. (The numerator is still read from
                // the A register.)
                let numerator = register_a;
                let denominator = 2_u32.pow(combo_operand);
                registers_c = numerator / denominator;
            }
            _ => unreachable!("invalid instruction: {}", instruction),
        }

        ip += 2;
    }
    Some(output.join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
