use advent_of_code::parse_u32;

advent_of_code::solution!(2);



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
                let mut prev = parse_u32(bytes, &mut i);

                while i < bytes.len() {
                    let c = parse_u32(bytes, &mut i);

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
    let mut count = 0;

    for line in input.lines() {
        let (correct, levels) = is_valid(line.as_bytes());

        if correct {
            count += 1;
            continue;
        }

        if (0..levels.len()).any(|i| is_valid_with_one_removed(&levels, i)) {
            count += 1;
        }
    }

    Some(count)
}

fn is_valid(bytes: &[u8]) -> (bool, Vec<u32>) {
    let mut increasing = false;
    let mut decreasing = false;

    let mut i = 0;

    let mut prev = parse_u32(bytes, &mut i);
    let mut levels = Vec::with_capacity(20);
    levels.push(0);

    let mut correct = true;

    // Use unsafe to directly access raw pointers for faster iteration
    while i < bytes.len() {
        let current = parse_u32(bytes, &mut i);
        levels.push(current);

        let diff = prev.abs_diff(current);
        if diff > 3 {
            correct = false; // Invalid difference
        }

        if prev < current {
            increasing = true;
        } else if prev > current {
            decreasing = true;
        }

        if increasing && decreasing {
            correct = false;
        }

        prev = current;
    }

    (correct, levels)
}

fn is_valid_with_one_removed(levels: &[u32], remove_index: usize) -> bool {
    let len = levels.len();
    if len < 2 {
        return true;
    }

    let mut increasing = false;
    let mut decreasing = false;

    // Use unsafe to directly access raw pointers for faster iteration
    unsafe {
        let mut prev: Option<u32> = None;

        for i in 0..len {
            if i == remove_index {
                continue; // Skip the element being "removed"
            }

            if let Some(prev_value) = prev {
                let current = *levels.get_unchecked(i);
                let diff = prev_value.abs_diff(current);
                if diff > 3 {
                    return false; // Invalid difference
                }

                if prev_value < current {
                    increasing = true;
                } else if prev_value > current {
                    decreasing = true;
                }

                if increasing && decreasing {
                    return false;
                }
            }

            prev = Some(*levels.get_unchecked(i));
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
