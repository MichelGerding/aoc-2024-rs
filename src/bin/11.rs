use memoize::memoize;

use rustc_hash::FxHashMap;
use rayon::prelude::*;

advent_of_code::solution!(11);

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default(), SharedCache)]
fn blink_stone(stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }
    if stone == 0 {
        let result = blink_stone(1, blinks - 1);
        return result;
    }

    let digit_count = ((stone + 1) as f64).log10().ceil() as u32;
    let result = if digit_count % 2 == 0 {
        let split = 10_u64.pow(digit_count / 2);
        blink_stone(stone / split, blinks - 1) + blink_stone(stone % split, blinks - 1)
    } else {
        blink_stone(stone * 2024, blinks - 1)
    };

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .par_split(' ')
            .map(|s| blink_stone(s.parse::<u64>().unwrap(), 25))
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .par_split(' ')
            .map(|s| blink_stone(s.parse::<u64>().unwrap(), 75))
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
