use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

advent_of_code::solution!(20);

fn explore(start: (i32, i32), free_spaces: &FxHashSet<(i32, i32)>) -> FxHashMap<(i32, i32), i32> {
    let mut distances = FxHashMap::with_capacity_and_hasher(free_spaces.len(), Default::default()); // Pre-allocate for fewer resizes
    let mut queue = VecDeque::with_capacity(free_spaces.len()); // Pre-allocate for fewer resizes

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some((cx, cy)) = queue.pop_front() {
        let current_distance = distances[&(cx, cy)];

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (cx + dx, cy + dy);

            // Only process unvisited and valid free spaces
            if free_spaces.contains(&np) && !distances.contains_key(&np) {
                distances.insert(np, current_distance + 1);
                queue.push_back(np);
            }
        }
    }

    distances
}

fn get_savings(distances: &FxHashMap<(i32, i32), i32>, jump_size: i32) -> usize {
    // Pre-calculate jump range once, with capacity hint
    let capacity = ((2 * jump_size + 1) * (2 * jump_size + 1)) as usize;
    let jump_range = {
        let mut ranges = Vec::with_capacity(capacity);
        for dx in -jump_size..=jump_size {
            for dy in -jump_size..=jump_size {
                if dx.abs() + dy.abs() <= jump_size {
                    ranges.push((dx, dy));
                }
            }
        }
        ranges
    };

    distances
        .into_par_iter()
        .map(|(&(px, py), &initial_cost)| {
            let mut count = 0u32;

            // Use reference to avoid copying
            for &(dx, dy) in &jump_range {
                let np = (px + dx, py + dy);

                // Avoid option handling with direct get
                if let Some(&np_cost) = distances.get(&np) {
                    let cheat_cost = dx.abs() + dy.abs();
                    count += u32::from(initial_cost - np_cost - cheat_cost >= 100);
                }
            }

            count as usize
        })
        .sum()
}

const FREE_SPACE_COUNT: usize = 9441;

fn parse_input(input: &str) -> (FxHashSet<(i32, i32)>, (i32, i32)) {
    let mut free_space =
        FxHashSet::with_capacity_and_hasher(FREE_SPACE_COUNT, Default::default());

    let mut start = (0, 0);
    let mut x = 0;
    let mut y = 0;

    for &c in input.as_bytes() {
        match c {
            b'#' => x += 1,
            b'\n' => {
                x = 0;
                y += 1;
            }
            _ => {
                free_space.insert((x, y));
                if c == b'S' {
                    start = (x, y);
                }
                x += 1;
            }
        }
    }

    (free_space, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (free_space, start) = parse_input(input);
    let distances = explore(start, &free_space);

    let savings = get_savings(&distances, 2);

    Some(savings as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (free_space, start) = parse_input(input);
    let distances = explore(start, &free_space);

    let savings = get_savings(&distances, 20);

    Some(savings as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
