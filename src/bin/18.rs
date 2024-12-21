use advent_of_code::{euclidean_distance, parse_unsigned, Grid};
use pathfinding::prelude::{astar};

advent_of_code::solution!(18);

fn solve(grid: &Grid, start: (u32, u32), end: (u32, u32)) -> Option<(Vec<(u32, u32)>, u32)> {
    astar(
        &start,
        |p| grid.next_moves_equal_weight(p.0, p.1, 1),
        |p| euclidean_distance(*p, end),
        |p| *p == end
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();
    // parse the bytes

    #[cfg(test)]
    const FALLING_BYTES: usize = 12;
    #[cfg(not(test))]
    const FALLING_BYTES: usize = 1024;

    let mut max_x = 0;
    let mut max_y = 0;

    let mut locations = [(0u32, 0u32); FALLING_BYTES];
    let mut idx = 0;
    let mut i = 0;

    while i < bytes.len() && idx < locations.len() {
        let x = parse_unsigned(bytes, &mut i);
        let y = parse_unsigned(bytes, &mut i);

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }

        locations[idx] = (x, y);
        idx += 1;
    }

    let grid = Grid::new(max_x + 1, max_y + 1, &locations);

    Some(solve(&grid, (0, 0), (max_x, max_y)).unwrap().1)
}

pub fn part_two(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    // parse the bytes

    #[cfg(test)]
    const FALLING_BYTES: usize = 25;
    #[cfg(test)]
    const GRID_SIZE: u32 = 6;
    #[cfg(test)]
    const SAFE_DROPS: usize = 11;

    #[cfg(not(test))]
    const FALLING_BYTES: usize = 3450;
    #[cfg(not(test))]
    const SAFE_DROPS: usize = 1024;
    #[cfg(not(test))]
    const GRID_SIZE: u32 = 70;

    let mut locations = [(0u32, 0u32); FALLING_BYTES];
    let mut idx = 0;
    let mut i = 0;

    while i < bytes.len() {
        let x = parse_unsigned(bytes, &mut i);
        let y = parse_unsigned(bytes, &mut i);

        locations[idx] = (x, y);
        idx += 1;
    }

    let mut left = SAFE_DROPS;
    let mut right = FALLING_BYTES + 1;
    let mut result = None;

    while left < right {
        let mid = (left + right) / 2;
        let grid = Grid::new(GRID_SIZE + 1, GRID_SIZE + 1, &locations[..mid]);

        let res = solve(&grid,(0, 0), (GRID_SIZE, GRID_SIZE));
        if res.is_none() {
            result = Some(mid);
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if let Some(idx) = result {
        return Some(format!("{},{}", locations[idx - 1].0, locations[idx - 1].1));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
