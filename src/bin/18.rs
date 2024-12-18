use advent_of_code::{euclidean_distance, parse_unsigned, Direction};
use pathfinding::prelude::{astar};

advent_of_code::solution!(18);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Grid {
    width: u32,
    height: u32,
    grid: Vec<bool>,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Grid {
    fn new(width: u32, height: u32, falling_memory: &[(u32, u32)]) -> Self {
        let mut grid = Self {
            width,
            height,
            grid: vec![false; (width * height) as usize],
        };

        falling_memory.iter().for_each(|(x, y)| {
            let offset = grid.convert_offset(*x, *y);
            grid.grid[offset] = true;
        });

        grid
    }

    fn safe_spaces(&self, x: u32, y: u32) -> Vec<((u32, u32), u32)> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let new_pos = (x as i32 + dir.delta().0, y as i32 + dir.delta().1);

                if new_pos.0 < 0
                    || new_pos.0 >= self.width as i32
                    || new_pos.1 < 0
                    || new_pos.1 >= self.height as i32
                {
                    return None;
                }

                let new_offset = self.convert_offset(new_pos.0 as u32, new_pos.1 as u32);
                if self.grid[new_offset] {
                    return None;
                }

                Some(((new_pos.0 as u32, new_pos.1 as u32), 1))
            })
            .collect::<Vec<_>>()
    }

    fn convert_offset(&self, x: u32, y: u32) -> usize {
        (y * (self.width) + x) as usize
    }

    fn insert(&mut self, x: u32, y: u32) {
        let offset = self.convert_offset(x, y);
        self.grid[offset] = true;
    }

    fn solve(&self, start: (u32, u32), end: (u32, u32)) -> Option<(Vec<(u32, u32)>, u32)> {
        astar(
            &start,
            |p| self.safe_spaces(p.0, p.1),
            |p| euclidean_distance(*p, end),
            |p| *p == end
        )
    }
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

    Some(grid.solve((0, 0), (max_x, max_y)).unwrap().1)
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

        let res = grid.solve((0, 0), (GRID_SIZE, GRID_SIZE));
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
