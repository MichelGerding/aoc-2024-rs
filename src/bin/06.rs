use std::collections::HashSet;
use std::hash::Hash;
use rayon::prelude::*;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum GuardDirections {
    UP, LEFT, RIGHT, DOWN,
}

impl GuardDirections {
    fn to_offset(&self) -> Position {
        match self {
            GuardDirections::UP => Position { y: -1, x: 0 },
            GuardDirections::LEFT => Position { y: 0, x: -1 },
            GuardDirections::RIGHT => Position { y: 0, x: 1 },
            GuardDirections::DOWN => Position { y: 1, x: 0 },
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            GuardDirections::UP => GuardDirections::RIGHT,
            GuardDirections::RIGHT => GuardDirections::DOWN,
            GuardDirections::DOWN => GuardDirections::LEFT,
            GuardDirections::LEFT => GuardDirections::UP,
        }
    }
}
#[derive(Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x: x as isize, y: y as isize }
    }

    fn add(&self, rhs: Self) -> Self {
        let xo = self.x + rhs.x;
        let yo = self.y + rhs.y;

        Position::new(xo as usize, yo as usize)
    }

    pub fn in_grid(&self, &(x_max, y_max): &(isize, isize)) -> bool {
        !(self.x < 0 || self.x >= x_max ||
            self.y < 0 || self.y >= y_max)
    }
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}


#[derive(Copy, Clone)]
struct Guard {
    pub guard_direction: GuardDirections,
    pub position: Position,
}




impl Guard {
    pub fn move_spot(&mut self, grid: &Vec<Vec<u8>>) {
        let bounds = (
            grid[0].len() as isize,
            grid.len() as isize,
        );

        let offset = self.guard_direction.to_offset();
        let np = &self.position.add(offset);

        if !np.in_grid(&bounds) {
            self.position = np.clone();
            return;
        }


        let c = grid[np.y as usize][np.x as usize];
        if c == b'.' {
            let n_offset = self.guard_direction.to_offset();
            let new_pos = self.position.add(n_offset);

            self.position = new_pos;
            return;
        }
        self.guard_direction = GuardDirections::rotate_right(&self.guard_direction);
    }

    pub fn check_loop(&self, grid: &[Vec<u8>], bounds: (isize, isize)) -> bool {
        let mut pos = self.position.clone();
        let mut dir = self.guard_direction.clone();
        let mut visited = HashSet::new();

        loop {
            if !visited.insert((pos, dir)) {
                return true;
            }

            let offset = dir.to_offset();
            let next_pos = pos.add(offset);

            if !next_pos.in_grid(&bounds) {
                return false;
            }

            if grid[next_pos.y as usize][next_pos.x as usize] == b'.' {
                pos = next_pos;
            } else {
                dir = dir.rotate_right();
            }
        }
    }

    pub fn get_possible_spots(&self, grid: &Vec<Vec<u8>>) -> HashSet<Position> {
        let mut guard = self.clone();
        let mut visited_spots = HashSet::new();

        let bounds = (
            grid[0].len() as isize,
            grid.len() as isize,
        );

        while guard.position.in_grid(&bounds) {
            visited_spots.insert(guard.position.clone());
            guard.move_spot(&grid);
        }

        visited_spots
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let (grid, mut guard) = parse_input(input);
    let mut visited_spots = HashSet::new();


    let bounds = (
        grid[0].len() as isize,
        grid.len() as isize,
    );

    while guard.position.in_grid(&bounds) {
        visited_spots.insert(guard.position.clone());
        guard.move_spot(&grid);
    }


    Some(visited_spots.len() as u32)
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Guard) {
    let mut guard: Option<Guard> = None;
    (input.lines().enumerate().map(|(y, row)| {
        row.bytes().enumerate().map(|(x, c)| {
            match c {
                b'^' | b'v' | b'>' | b'<' => {
                    guard = Some(Guard {
                        position: Position::new(x, y),
                        guard_direction: match c {
                            b'^' => GuardDirections::UP,
                            b'v' => GuardDirections::DOWN,
                            b'>' => GuardDirections::RIGHT,
                            b'<' => GuardDirections::LEFT,
                            _ => unreachable!(),
                        },
                    });
                    b'.'
                }
                other => other,
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>(), guard.unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, guard) = parse_input(input);

    let bounds = (
        grid[0].len() as isize,
        grid.len() as isize,
    );

    Some(guard.get_possible_spots(&grid)
        .par_iter()
        .filter(|&pos| {
            let mut g = grid.clone(); // Clone the grid for thread safety

            if g[pos.y as usize][pos.x as usize] != b'.' {
                return false;
            }

            g[pos.y as usize][pos.x as usize] = b'O';
            let looped = guard.check_loop(&g, bounds);

            looped
        })
        .count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
