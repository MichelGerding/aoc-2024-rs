#![feature(maybe_uninit_uninit_array)]

#[cfg(test)]
const GRID_HEIGHT: i32 = 7;
#[cfg(test)]
const GRID_WIDTH: i32 = 11;
#[cfg(test)]
const MAX_ROBOTS: usize = 12;

#[cfg(not(test))]
const GRID_HEIGHT: i32 = 103;
#[cfg(not(test))]
const GRID_WIDTH: i32 = 101;
#[cfg(not(test))]
const MAX_ROBOTS: usize = 500;

use std::mem::MaybeUninit;
use std::ptr;

advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Bound {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bound {
    fn contains(&self, r: &Robot) -> bool {
        let (x, y) = r.pos;
        x >= self.min_x && y >= self.min_y && x < self.max_x && y < self.max_y
    }
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn wait_secs(&self, seconds: i32, width: i32, height: i32) -> Self {
        // move the robot
        let x_vel = self.velocity.0 * seconds + self.pos.0;
        let y_vel = self.velocity.1 * seconds + self.pos.1;

        // get the position with wrapping around the grid
        let mut x_moved = x_vel % width;
        let mut y_moved = y_vel % height;

        if x_moved < 0 {
            x_moved = width + x_moved
        }

        if y_moved < 0 {
            y_moved = height + y_moved
        }

        Robot {
            pos: (x_moved, y_moved),
            velocity: self.velocity.clone(),
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            pos: (0, 0),
            velocity: (0, 0),
        }
    }
}

const QUADRANTS: [Bound; 4] = [
    Bound {
        min_x: 0,
        min_y: 0,
        max_x: GRID_WIDTH / 2,
        max_y: GRID_HEIGHT / 2,
    },
    Bound {
        min_x: GRID_WIDTH / 2 + 1,
        min_y: 0,
        max_x: GRID_WIDTH,
        max_y: GRID_HEIGHT / 2,
    },
    Bound {
        min_x: 0,
        min_y: GRID_HEIGHT / 2 + 1,
        max_x: GRID_WIDTH / 2,
        max_y: GRID_HEIGHT,
    },
    Bound {
        min_x: GRID_WIDTH / 2 + 1,
        min_y: GRID_HEIGHT / 2 + 1,
        max_x: GRID_WIDTH,
        max_y: GRID_HEIGHT,
    },
];

pub fn get_x_y(part: &str) -> (i32, i32) {
    // Split once by the comma, then parse each part (x, y) directly
    let (x_str, y_str) = part.split_once(',').unwrap();
    let x = x_str.parse::<i32>().unwrap();
    let y = y_str.parse::<i32>().unwrap();

    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut quadrants = [0; 4];

    let bytes = input.as_bytes();

    let mut i = 2;

    let mut robot_count = 0;
    while robot_count < MAX_ROBOTS {
        let robot = Robot::wait_secs(&parse_bot(bytes, &mut i), 100, GRID_WIDTH, GRID_HEIGHT);

        if QUADRANTS[0].contains(&robot) {
            quadrants[0] += 1;
        }
        if QUADRANTS[1].contains(&robot) {
            quadrants[1] += 1;
        }
        if QUADRANTS[2].contains(&robot) {
            quadrants[2] += 1;
        }
        if QUADRANTS[3].contains(&robot) {
            quadrants[3] += 1;
        }
        robot_count += 1;
    }

    Some(quadrants.iter().fold(1, |acc, quadrant| acc * quadrant))
}

use rayon::prelude::*;
use advent_of_code::parse_i32;

fn parse_bot(bytes: &[u8], i: &mut usize) -> Robot {
    let pos_x = parse_i32(&bytes, i);
    let pos_y = parse_i32(&bytes, i);

    *i += 2;
    let vel_x = parse_i32(&bytes, i);
    let vel_y = parse_i32(&bytes, i);
    *i += 2;

    Robot {
        pos: (pos_x, pos_y),
        velocity: (vel_x, vel_y),
    }
}



pub fn part_two(input: &str) -> Option<i32> {
    #[allow(invalid_value)]
    let mut robots: [Robot; MAX_ROBOTS] = unsafe { MaybeUninit::uninit().assume_init() }; // Uninitialized array
    let mut robot_count = 0;

    let bytes = input.as_bytes();

    let mut i = 2;
    while robot_count < MAX_ROBOTS {
        let robot = parse_bot(bytes, &mut i);

        unsafe {
            ptr::write(&mut robots[robot_count], robot);
        }
        robot_count += 1;
    }

    (0..5000).into_par_iter().find_map_first(|i| {
        let mut seen_positions = [0u128; GRID_WIDTH as usize];
        let mut found = true;

        for robot in &robots {
            unsafe {
                let (x, y) = robot.wait_secs(i * 2, GRID_WIDTH, GRID_HEIGHT).pos;
                let seen_bits_ptr = seen_positions.as_mut_ptr().add(x as usize);
                let bitmask = 1 << y as usize;
                if (*seen_bits_ptr & bitmask) != 0 {
                    found = false;
                    break;
                }

                *seen_bits_ptr |= bitmask
            }
        }

        if found {
            return Some(i * 2);
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
