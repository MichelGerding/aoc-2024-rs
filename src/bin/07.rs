advent_of_code::solution!(7);

use rayon::prelude::*;

fn can_produce_target(target: u64, numbers: &[u64]) -> bool {
    unsafe {
        let n = numbers.len();
        let mut prev = vec![numbers[0]];

        for i in 1..n - 1 {
            let mut current = Vec::with_capacity(prev.len() * 2);
            let n = numbers.get_unchecked(i);

            for &value in &prev {
                let sum = value + n;
                let product = value * n;

                if sum <= target { current.push(sum); }
                if product <= target { current.push(product); }
            }

            prev = current;
        }

        let n = numbers.get_unchecked(n - 1);
        for &value in &prev {
            if value + n == target || value * n == target {
                return true;
            }
        }

        false
    }
}


fn can_produce_target_concat(target: u64, numbers: &[u64]) -> bool {
    unsafe {
        let n = numbers.len();
        let mut prev = vec![numbers[0]];

        for i in 1..n - 1 {
            let mut current = Vec::with_capacity(prev.len() * 2);
            let n = numbers.get_unchecked(i);

            for &value in &prev {
                let sum = value + n;
                let product = value * n;
                let concatenated = fast_concatenate(value, *n);

                if sum <= target { current.push(sum); }
                if product <= target { current.push(product); }
                if concatenated <= target { current.push(concatenated); }
            }

            prev = current;
        }

        let n = numbers.last().unwrap_unchecked();
        for &value in &prev {
            if value + n == target || value * n == target || fast_concatenate(value, *n) == target {
                return true;
            }
        }
        false
    }
}
pub fn fast_concatenate(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.par_lines()
            .filter_map(|line| {
                unsafe {
                    if line.is_empty() {
                        return None;
                    }
                    let mut parts = line.splitn(2, ':');
                    let target = parts.next().unwrap_unchecked().parse::<u64>().unwrap_unchecked();
                    let nums_str = parts.next().unwrap_unchecked();

                    let nums = nums_str
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap_unchecked())
                        .collect::<Vec<u64>>();

                    Some((target, nums))
                }
            }).filter(|(result, nums)| can_produce_target(*result, nums)).map(|(result, _)| result).sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input.par_lines()
            .filter_map(|line| {
                unsafe {
                    if line.is_empty() {
                        return None;
                    }
                    let mut parts = line.splitn(2, ':');
                    let target = parts.next().unwrap_unchecked().parse::<u64>().unwrap_unchecked();
                    let nums_str = parts.next().unwrap_unchecked();

                    let nums = nums_str
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap_unchecked())
                        .collect::<Vec<u64>>();

                    Some((target, nums))
                }
            }).filter(|(result, nums)| can_produce_target_concat(*result, nums)).map(|(result, _)| result).sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
