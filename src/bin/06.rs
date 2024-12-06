use std::collections::HashSet;
use std::hash::Hash;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Eq, PartialEq)]
enum GuardDirections {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl GuardDirections {
    pub fn to_offset(&self) -> Position {
        match self {
            GuardDirections::UP => Position{ y: -1,x: 0},
            GuardDirections::LEFT => Position{ y: 0,x: -1},
            GuardDirections::RIGHT => Position{ y: 0,x: 1},
            GuardDirections::DOWN => Position{ y: 1, x: 0},
        }
    }

    pub fn rotate_right(direction: &GuardDirections) -> Self {
        match direction {
            GuardDirections::UP => GuardDirections::RIGHT,
            GuardDirections::RIGHT => GuardDirections::DOWN,
            GuardDirections::DOWN => GuardDirections::LEFT,
            GuardDirections::LEFT => GuardDirections::UP,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            GuardDirections::UP => String::from("^"),
            GuardDirections::LEFT => String::from("<"),
            GuardDirections::RIGHT => String::from(">"),
            GuardDirections::DOWN => String::from("-"),
        }
    }
}

impl Hash for GuardDirections {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_string().hash(state);
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
        let xo= self.x + rhs.x;
        let yo= self.y + rhs.y;

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

    pub fn check_loop(&self, grid: &Vec<Vec<u8>>) -> bool {
        let mut g = self.clone();

        let mut visited_spots: HashSet<(Position, GuardDirections)> = HashSet::new();
        let bounds  = (
            grid[0].len() as isize,
            grid.len() as isize,
        );

        while g.position.in_grid(&bounds) {
            if visited_spots.contains(&(g.position, g.guard_direction)) {
               return true;
            }

            visited_spots.insert((g.position.clone(), g.guard_direction.clone()));
            g.move_spot(&grid);
        }

        false
    }

    pub fn move_spot(&mut self, grid: &Vec<Vec<u8>>) {
        let bounds  = (
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
            return
        }
        self.guard_direction = GuardDirections::rotate_right(&self.guard_direction);
    }
}

pub fn print_grid(grid: &[Vec<u8>], guard: &Guard) {
    let gp = guard.position;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if gp.x == x as isize && gp.y == y as isize {
                print!("{}", guard.guard_direction.as_string());
            } else {
              print!("{}", grid[y][x] as char);
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard: Option<Guard> = None;

    let grid = input.lines().enumerate().map(|(y, row)| {
        row.bytes().enumerate().map(|(x, c)| {
            match c {
                b'^' => {
                    guard = Some(Guard {
                        position: Position::new(x, y),
                        guard_direction: GuardDirections::UP
                    });
                    b'.'
                    // break;
                }
                b'v' => {
                    guard = Some(Guard {
                        position: Position::new(x, y),
                        guard_direction: GuardDirections::DOWN
                    });
                    b'.'
                    // break;
                }
                b'>' => {
                    guard = Some(Guard {
                        position: Position::new(x, y),
                        guard_direction: GuardDirections::RIGHT
                    });
                    b'.'
                    // break;
                }
                b'<' => {
                    guard = Some(Guard {
                        position: Position::new(x, y),
                        guard_direction: GuardDirections::LEFT
                    });
                    b'.'
                    // break;
                }
                v => v,
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();


    let mut guard = guard.unwrap();
    let mut visited_spots = HashSet::new();


    let bounds  = (
        grid[0].len() as isize,
        grid.len() as isize,
    );
    // while guard.position.in_grid(&bounds) {
    while guard.position.in_grid(&bounds) {
        visited_spots.insert(guard.position.clone());
        guard.move_spot(&grid);
    }


    Some(visited_spots.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard: Option<Guard> = None;

    let mut grid = input.lines().enumerate().map(|(y, row)| {
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
    }).collect::<Vec<Vec<u8>>>();


    let guard = guard.unwrap();
    let mut loops = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            // set a single spot to an obstacle
            if grid[y][x] != b'.' {
                continue;
            }
            grid[y][x] = b'O';

            if guard.check_loop(&grid) {
                loops += 1;
            }
            grid[y][x] = b'.';
        }
    }

    Some(loops)
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
