use ahash::AHashMap;
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

advent_of_code::solution!(11);

fn _blink(mut stones: Vec<u64>, amount: usize) -> Option<usize> {
    for _ in 0..amount {
        let mut new_stones = Vec::new();
        stones.iter_mut().for_each(|stone| {
            if *stone == 0 {
                *stone = 1;
                return;
            }

            let digit_count = stone.to_string().len();
            if digit_count % 2 == 0 {
                let split = 10_u64.pow((digit_count / 2) as u32);
                let left = *stone / split;
                let right = *stone % split;

                *stone = left;
                new_stones.push(right);
            } else {
                *stone *= 2024;
            }
        });

        stones.extend(new_stones);
    }

    Some(stones.len())
}


static CACHE: Lazy<Mutex<AHashMap<(u64, usize), usize>>> =
    Lazy::new(|| Mutex::new(AHashMap::new()));

fn blink_stone(stone: u64, blinks: usize) -> usize {
    // Check if the result is already cached
    let cache_key = (stone, blinks);
    if let Some(&cached_result) = CACHE.lock().unwrap().get(&cache_key) {
        return cached_result;
    }

    // Base cases
    if blinks == 0 {
        return 1;
    }
    if stone == 0 {
        let result = blink_stone(1, blinks - 1);
        CACHE.lock().unwrap().insert(cache_key, result);
        return result;
    }

    let digit_count = ((stone + 1) as f64).log10().ceil() as u32;
    let result = if digit_count % 2 == 0 {
        let split = 10_u64.pow(digit_count / 2);
        blink_stone(stone / split, blinks - 1) + blink_stone(stone % split, blinks - 1)
    } else {
        blink_stone(stone * 2024, blinks - 1)
    };

    // Cache the result
    CACHE.lock().unwrap().insert(cache_key, result);
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
        assert_eq!(result, Some(55312));
    }
}
