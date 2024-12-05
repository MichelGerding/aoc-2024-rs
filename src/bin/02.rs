#![feature(array_windows)]
advent_of_code::solution!(2);

fn get_num(bytes: &[u8], idx: &mut usize) -> u32 {
    let mut c = 0;
    while idx < &mut bytes.len() && !bytes[*idx].is_ascii_whitespace() {
        c = c * 10 + (bytes[*idx] - b'0') as u32;
        *idx += 1;
    }

    *idx += 1;

    c
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let mut increasing = false;
                let mut decreasing = false;

                let bytes = line.as_bytes();
                // pars the first number
                let mut i = 0;
                let mut prev = get_num(bytes, &mut i);

                while i < bytes.len() {
                    let c = get_num(bytes, &mut i);

                    let diff = c.abs_diff(prev);
                    if !(1..=3).contains(&diff) {
                        return false;
                    }

                    if c < prev {
                        increasing = true;
                    } else if c > prev {
                        decreasing = true;
                    }

                    if increasing && decreasing {
                        return false;
                    }

                    prev = c;
                }

                true
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .into_iter()
                    .map(|c| c.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .filter(|levels| {
                if save_levels(levels) {
                    return true;
                }

                (0..levels.len())
                    .find(|i| {
                        let mut adjusted_level = levels.clone();
                        adjusted_level.remove(*i);

                        save_levels(&adjusted_level)
                    })
                    .is_some()
            })
            .count() as u32,
    )
}

fn save_levels(levels: &Vec<u8>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    for [a, b] in levels.array_windows() {
        let diff = a.abs_diff(*b);
        if !(1..=3).contains(&diff) {
            return false;
        }

        if a < b {
            increasing = true;
        } else if a > b {
            decreasing = true;
        }

        if increasing && decreasing {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
