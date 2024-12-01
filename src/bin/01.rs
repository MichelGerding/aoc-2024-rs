use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = BinaryHeap::with_capacity(1000);
    let mut right = BinaryHeap::with_capacity(1000);

    input.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }

        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    });

    Some(
        left.into_sorted_vec()
            .iter()
            .zip(right.into_sorted_vec().iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::with_capacity(1000);
    let mut right: HashMap<u32, u32> = HashMap::with_capacity(1000);

    input.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }

        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());

        let r = parts.next().unwrap().parse::<u32>().unwrap();
        *right.entry(r).or_insert(0) += r;
    });

    Some(
        left.into_iter()
            .map(|v| *right.entry(v).or_default())
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1222801u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(22545250u32));
    }
}
