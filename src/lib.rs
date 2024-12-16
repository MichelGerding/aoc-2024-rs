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
}

