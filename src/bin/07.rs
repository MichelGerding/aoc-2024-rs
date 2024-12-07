advent_of_code::solution!(7);

use rayon::prelude::*;
use ahash::AHashSet;
fn can_produce_target(target: u64, numbers: &[u64]) -> bool {
    let n = numbers.len();

    let mut prev = AHashSet::with_capacity(n);
    prev.insert(numbers[0]);

    for i in 1..n {
        let mut current = AHashSet::with_capacity(prev.len() * 2);

        for &value in &prev {
            let sum = value + numbers[i];
            let product = value * numbers[i];
            let concatenated = fast_concatenate(value, numbers[i]);

            if sum <= target { current.insert(sum); }
            if product <= target { current.insert(product); }
            if concatenated <= target { current.insert(concatenated); }
        }

        prev = current;
    }

    prev.contains(&target)
}


fn can_produce_target_concat(target: u64, numbers: &[u64]) -> bool {
    let n = numbers.len();

    let mut prev = AHashSet::with_capacity(n);
    prev.insert(numbers[0]);

    for i in 1..n {
        let mut current = AHashSet::with_capacity(prev.len() * 2);

        for &value in &prev {
            let sum = value + numbers[i];
            let product = value * numbers[i];
            let concatenated = fast_concatenate(value, numbers[i]);

            if sum <= target { current.insert(sum); }
            if product <= target { current.insert(product); }
            if concatenated <= target { current.insert(concatenated); }
        }

        prev = current;
    }

    prev.contains(&target)
}
pub fn fast_concatenate(a: u64, b: u64) -> u64 {
    unsafe {
        let b_digits = if b == 0 { 1 } else { (b as f64).log10().floor() as u32 + 1 };
        let multiplier = 10u64.pow(b_digits);
        a.unchecked_mul(multiplier).unchecked_add(b)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines() // Process lines in parallel
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                }

                // Use manual splitting for efficiency
                let mut parts = line.splitn(2, ':');
                let result = parts.next()?.parse::<u64>().ok()?;
                let nums_str = parts.next()?;

                // Parse numbers using unsafe for performance
                let nums = unsafe {
                    nums_str
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap_unchecked())
                };

                Some((result, nums.collect::<Vec<u64>>()))
            })
            .filter(|(result, nums)| can_produce_target(*result, nums))
            .map(|(result, _)| result)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines() // Process lines in parallel
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                }

                // Use manual splitting for efficiency
                let mut parts = line.splitn(2, ':');
                let result = parts.next()?.parse::<u64>().ok()?;
                let nums_str = parts.next()?;

                // Parse numbers using unsafe for performance
                let nums = unsafe {
                    nums_str
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap_unchecked())
                };

                Some((result, nums.collect::<Vec<u64>>()))
            })
            .filter(|(result, nums)| can_produce_target_concat(*result, nums))
            .map(|(result, _)| result)
            .sum::<u64>(),
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
