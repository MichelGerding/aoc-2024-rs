advent_of_code::solution!(19);

use dashmap::DashMap;
use rayon::prelude::*;
use rustc_hash::{FxHashSet};

fn solve_design<'a>(
    design: &'a str,
    towels: &FxHashSet<String>,
    memo: &DashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(cached_result) = memo.get(&design) {
        return *cached_result;
    }

    let mut possibilities = 0;
    for towel in towels {
        if &design == towel {
            possibilities += 1;
            continue;
        }

        if design.starts_with(towel) {
            let result = solve_design(&design[towel.len()..], towels, memo);
            if result != 0 {
                possibilities += result;
            }
        }
    }

    memo.insert(&design, possibilities);
    possibilities
}

fn parse_input(input: &str) -> (FxHashSet<String>, Vec<String>) {
    let mut parts = input.splitn(2, "\n\n");

    let mut towels = FxHashSet::default();

    parts.next().unwrap().lines().for_each(|line| {
        line.split(", ").for_each(|towel| {
            towels.insert(towel.to_string());
        });
    });

    let designs = parts
        .next()
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    (towels, designs)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, designs) = parse_input(input);
    let memo = DashMap::new();

    Some(
        designs
            .par_iter()
            .filter(|&design| solve_design(design, &towels, &memo) != 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, designs) = parse_input(input);
    let memo = DashMap::new();

    Some(
        designs
            .par_iter()
            .filter_map(|design| {
                let count = solve_design(design, &towels, &memo);
                if count != 0 {
                    Some(count)
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
