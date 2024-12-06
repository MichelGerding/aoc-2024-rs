use rayon::prelude::*;
use std::hash::Hash;

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq, Hash)]
enum GuardDirections {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl GuardDirections {
    fn to_offset(&self) -> i32 {
        match self {
            GuardDirections::UP => -(GRID_SIZE as i32 + 1), //Position { y: -1, x: 0 },
            GuardDirections::LEFT => -1,                    //Position { y: 0, x: -1 },
            GuardDirections::RIGHT => 1,                    //Position { y: 0, x: 1 },
            GuardDirections::DOWN => GRID_SIZE as i32 + 1,  //Position { y: 1, x: 0 },
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
struct Guard {
    pub direction: GuardDirections,
    pub position: i32,
}

#[cfg(not(debug_assertions))]
const GRID_SIZE: usize = 130;
#[cfg(not(debug_assertions))]
const INITIAL_POSITION: i32 = (84 * (GRID_SIZE + 1) + 89) as i32;

#[cfg(debug_assertions)]
const GRID_SIZE: usize = 10;
#[cfg(debug_assertions)]
const INITIAL_POSITION: i32 = (6 * (GRID_SIZE + 1) + 4) as i32;

fn guard_move(bytes: &[u8]) -> Option<(i32, [u8; GRID_SIZE * GRID_SIZE + GRID_SIZE + 1])> {
    let mut visited = [0u8; GRID_SIZE * GRID_SIZE + GRID_SIZE + 1];
    let mut count = 0;

    // guard starts at INITIAL_POSITION going up
    let mut guard = Guard {
        direction: GuardDirections::UP,
        position: INITIAL_POSITION,
    };

    loop {
        unsafe {
            let offset = guard.direction.to_offset();
            let np = guard.position + offset;

            if np < 0 || np > ((GRID_SIZE + 1) * GRID_SIZE) as i32 {
                if *visited.get_unchecked(guard.position as usize) == 0 {
                    count += 1;
                    *visited.get_unchecked_mut(guard.position as usize) = 1;
                }

                break;
            }

            let c = *bytes.get_unchecked(np as usize);

            if c == b'\n' {
                if *visited.get_unchecked(guard.position as usize) == 0 {
                    count += 1;
                    *visited.get_unchecked_mut(guard.position as usize) = 1;
                }

                break;
            }

            if c != b'#' {
                let n_offset = guard.direction.to_offset();
                let new_pos = guard.position + n_offset;

                if *visited.get_unchecked(guard.position as usize) == 0 {
                    count += 1;
                    *visited.get_unchecked_mut(guard.position as usize) = 1;
                }

                guard.position = new_pos;
                continue;
            }
        }

        guard.direction = GuardDirections::rotate_right(&guard.direction);
    }

    Some((count, visited))
}

pub fn part_one(bytes: &str) -> Option<u32> {
    let (count, _) = guard_move(bytes.as_bytes())?;
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();

    let (_, visited) = guard_move(bytes)?;

    let to_check: Vec<usize> = visited
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if v == 0 {
                return None;
            }

            Some(i)
        })
        .collect();

    let loops = to_check
        .par_iter()
        .filter(|&&cell| {
            let mut guard = Guard {
                direction: GuardDirections::UP,
                position: INITIAL_POSITION,
            };

            let max = GRID_SIZE * GRID_SIZE;
            let mut i = 0;
            loop {
                if i > max {
                    return true;
                }
                i += 1;

                unsafe {
                    let offset = guard.direction.to_offset();
                    let np = guard.position + offset;

                    if np == cell as i32 {
                        guard.direction = GuardDirections::rotate_right(&guard.direction);
                        continue;
                    }

                    if np < 0 || np > ((GRID_SIZE + 1) * GRID_SIZE) as i32 {
                        return false;
                    }

                    let c = *bytes.get_unchecked(np as usize);

                    if c == b'\n' {
                        return false;
                    }

                    if c != b'#' {
                        let n_offset = guard.direction.to_offset();
                        let new_pos = guard.position + n_offset;

                        guard.position = new_pos;
                        continue;
                    }
                }

                guard.direction = GuardDirections::rotate_right(&guard.direction);
            }
        })
        .count() as u32;

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
