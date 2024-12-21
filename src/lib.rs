use std::hash::Hash;
use std::ops::{Add, Div, DivAssign, Mul, Rem};


pub mod template;

// Use this file to add helper functions and additional modules.

#[inline(always)]
pub fn convert_offset(x: i32, y: i32, grid_width: i32) -> usize {
    (y * (grid_width + 1) + x) as usize // add one for newlines
}

#[allow(dead_code)]
pub fn draw_grid(bytes: &[u8], grid_width: i32, grid_height: i32) {
    for y in 0..grid_height {
        for x in 0..grid_width {
            print!("{}", bytes[convert_offset(x, y, grid_width)] as char);
        }
        println!();
    }
}

pub fn parse_i32(buff: &[u8], i: &mut usize) -> i32 {
    let mut n = 0i32;
    let mut positive = true;
    while *i < buff.len() {
        let c = buff[*i];
        if c == b'-' {
            positive = false;
            *i += 1;
            continue;
        }

        if !c.is_ascii_digit() {
            break;
        }

        n = n * 10 + (c - b'0') as i32;
        *i += 1;
    }
    *i += 1;

    if positive {
        n
    } else {
        n * -1
    }
}

pub fn parse_u32(bytes: &[u8], idx: &mut usize) -> u32 {
    unsafe {
        let mut c = 0;
        while idx < &mut bytes.len() && bytes.get_unchecked(*idx).is_ascii_digit() {
            c = c * 10 + (bytes.get_unchecked(*idx) - b'0') as u32;
            *idx += 1;
        }

        *idx += 1;

        c
    }
}


pub fn parse_unsigned<T>(bytes: &[u8], idx: &mut usize) -> T
where
    T: From<u8> + Copy + Default + Add<Output = T> + Mul<Output = T> + 'static, // T must support Add and Mul
{
    unsafe {
        let mut result = T::default();
        while *idx < bytes.len() && bytes.get_unchecked(*idx).is_ascii_digit() {
            let digit = bytes.get_unchecked(*idx) - b'0';
            result = result * T::from(10u8) + T::from(digit);
            *idx += 1;
        }

        *idx += 1; // Move past any non-digit character (like a space or delimiter)

        result
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, )]
pub enum Direction {
    North,
    East,
    South,
    West,
    Any
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => *self
        }
    }

    pub fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            _ => *self
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            _ => (0, 0)
        }
    }

    pub fn offset(&self, width: usize) -> i32 {
        match self {
            Direction::North => -(width as i32),
            Direction::East => 1,
            Direction::South => width as i32,
            Direction::West => -1,
            _ => 0
        }
    }
}

pub fn divide_range(start: i64, end: i64, n: i64) -> Vec<(i64, i64)> {
    if n <= 0 {
        return vec![];
    }

    let range_size = (end - start) as f64;
    let chunk_size = (range_size / n as f64).ceil() as i64;

    (0..n)
        .map(|i| {
            let chunk_start = start + (i * chunk_size);
            let chunk_end = (start + ((i + 1) * chunk_size)).min(end);
            (chunk_start, chunk_end)
        })
        .collect()
}

pub fn compare_vecs(a: &Vec<u8>, b: &Vec<u8>) -> i64 {
    a.iter()
        .enumerate()
        .map(|(v, i)| {
            if b.get(v) == Some(i) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

pub trait Digits<T: From<u64>> {
    fn digits(self) -> impl Iterator<Item = T> where Self: Sized + PartialOrd<T> + Div<Output = T> + Rem<Output = T> + DivAssign + Copy, Self: From<u64> {
        let mut x = self;
        std::iter::from_fn(move || {
            if x > 0_u64.into() {
                let digit = x % 10_u64.into();
                x /= 10_u64.into();
                Some(digit)
            } else {
                None
            }
        })
    }
}

impl Digits<u64> for u64 {}


#[inline(always)]
pub fn euclidean_distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    ((a.0 as i32 - b.0 as i32).pow(2) + (a.1 as i32 - b.1 as i32).pow(2)) as u32
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<bool>,
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Grid {
    pub fn new(width: u32, height: u32, obstacle_locations: &[(u32, u32)]) -> Self {
        let mut grid = Self {
            width,
            height,
            grid: vec![false; (width * height) as usize],
        };

        obstacle_locations.iter().for_each(|(x, y)| {
            let offset = grid.convert_offset(*x, *y);
            grid.grid[offset] = true;
        });

        grid
    }

    pub fn next_moves(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
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

                Some((new_pos.0 as u32, new_pos.1 as u32))
            })
            .collect::<Vec<_>>()
    }

    pub fn next_moves_equal_weight(&self, x: u32, y: u32, weight: u32) -> Vec<((u32, u32), u32)> {
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

                Some(((new_pos.0 as u32, new_pos.1 as u32), weight))
            })
            .collect::<Vec<_>>()
    }


    pub fn convert_offset(&self, x: u32, y: u32) -> usize {
        (y * (self.width) + x) as usize
    }

    pub fn insert(&mut self, x: u32, y: u32) {
        let offset = self.convert_offset(x, y);
        self.grid[offset] = true;
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.grid[self.convert_offset(x, y)] { '#' } else { '.' });
            }
            println!();
        }
    }

}

pub fn manhatten_distance(a: &(u32, u32), b: &(u32, u32)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}
