advent_of_code::solution!(15);


macro_rules! generate_translations {
    () => {{
        const fn generate() -> [(i32, i32); 255] {
            let mut m = [(0,0); 255];
            m[b'^' as usize] = (0, -1);
            m[b'v' as usize] = (0, 1);
            m[b'<' as usize] = (-1, 0);
            m[b'>' as usize] = (1, 0);
            m
        }
        generate()
    }};
}


const TRANSLATIONS: [(i32, i32); 255] = generate_translations!();

#[inline(always)]
fn convert_offset(x: i32, y: i32, grid_width: i32) -> usize {
    (y * (grid_width + 1) + x) as usize // add one for newlines
}

fn score_grid(bytes: &[u8], grid_size: i32) -> u32 {
    let mut sum = 0;

    for y in 0..grid_size {
        for x in 0..grid_size {
            if bytes[convert_offset(x, y, grid_size)] == b'O' {
                sum += (100 * y + x) as u32;
            }
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    #[cfg(test)]
    const GRID_SIZE: i32 = 10;

    #[cfg(not(test))]
    const GRID_SIZE: i32 = 50;

    let mut s = input.to_string();
    let bytes = unsafe { s.as_bytes_mut() };

    let mut current_pos: (i32, i32) = (GRID_SIZE as i32 / 2 - 1, (GRID_SIZE as i32 / 2 - 1));

    let mut moves = (GRID_SIZE * GRID_SIZE + GRID_SIZE + 1) as usize;
    bytes[convert_offset(current_pos.0, current_pos.1, GRID_SIZE)] = b'.';

    while moves < bytes.len() {
        let m = bytes[moves];
        moves += 1;

        if m == b'\n' {
            continue;
        }

        let t = TRANSLATIONS[m as usize];

        if can_move(bytes, current_pos, t, GRID_SIZE, GRID_SIZE) {
            do_move(bytes, current_pos, t, GRID_SIZE, GRID_SIZE);

            current_pos = (current_pos.0 + t.0, current_pos.1 + t.1);
        } else {
            if bytes[moves] == m {
                moves += 1;
            }
        }
    }

    Some(score_grid(bytes, GRID_SIZE))
}

pub fn transform_input(input: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(input.len() * 2);
    for ch in input.chars() {
        match ch {
            '#' => bytes.extend_from_slice(b"##"),
            'O' => bytes.extend_from_slice(b"[]"),
            '.' => bytes.extend_from_slice(b".."),
            '@' => bytes.extend_from_slice(b".."),
            _ => bytes.extend_from_slice(ch.to_string().as_bytes()),
        }
    }

    bytes
}

pub fn part_two(input: &str) -> Option<i32> {
    #[cfg(test)]
    const GRID_HEIGHT: i32 = 10;

    #[cfg(not(test))]
    const GRID_HEIGHT: i32 = 50;
    const GRID_WIDTH: i32 = GRID_HEIGHT * 2;

    // transform the input
    let bytes = &mut transform_input(input);

    // initial position of the robot. it is slightly offset from the center due to the transformation
    // this moves it 1 to the left. The indexing is also done from 0 so we always need to subtract one
    let mut current_pos = (
        GRID_WIDTH / 2 - 2,
        GRID_HEIGHT / 2 - 1,
    );

    // the moves start after the grid.
    let mut moves = (GRID_HEIGHT * GRID_WIDTH + GRID_HEIGHT + 1) as usize;
    while moves < bytes.len() {
        // for _ in 0..1 {
        let m = bytes[moves];
        moves += 1;

        // a newline is an invalid instruction. so we will skip it
        if m == b'\n' {
            continue;
        }

        let direction = TRANSLATIONS[m as usize];
        let will_move = can_move(bytes, current_pos, direction, GRID_WIDTH, GRID_HEIGHT);

        if will_move {
            do_move(bytes, current_pos, direction, GRID_WIDTH, GRID_HEIGHT);
            current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        } else {
            if bytes[moves] == m {
                moves += 1;
            }
        }
    }

    // get the postition of the left most part of the crate
    let mut crate_postions = Vec::new();
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let c = bytes[convert_offset(x, y, GRID_WIDTH)];
            if c == b'[' {
                crate_postions.push((x, y));
            }
        }
    }

    Some(
        crate_postions
            .iter()
            .map(|crate_pos| {
                let (x, y) = *crate_pos;
                return 100 * y + x;
            })
            .sum(),
    )
}
fn can_move(
    grid: &mut [u8],
    curr_pos: (i32, i32),
    direction: (i32, i32),
    grid_width: i32,
    grid_height: i32,
) -> bool {
    let next_x = curr_pos.0 + direction.0;
    let next_y = curr_pos.1 + direction.1;

    // Check if the next position is out of bounds (without bounds checking in each step)
    if next_x <= 0 || next_y <= 0 || next_x > grid_width || next_y > grid_height {
        return false;
    }

    // Calculate the offset directly using unsafe to avoid bounds checking
    let offset = (next_y * (grid_width + 1) + next_x) as usize;
    let next_c = unsafe { *grid.get_unchecked(offset) };

    match (next_c, direction.0) {
        (b'#', _) => false, // Wall, can't move
        (b'.', _) => true,  // Empty space, can move
        (b'[', 0) | (b']', 0) => {
            // Brackets: Horizontal movement checks
        let horizontal_offset = if next_c == b'[' { 1 } else { -1 };
            // Combine checks for the current position and the neighboring bracket
            can_move(grid, (next_x, next_y), direction, grid_width, grid_height)
                && can_move(
                grid,
                (next_x + horizontal_offset, next_y),
                direction,
                grid_width,
                grid_height,
            )
        }
        // it is not a wall so it should be a small box or a horizontal movement
        _ => can_move(grid, (next_x, next_y), direction, grid_width, grid_height),
    }
}

fn do_move(
    grid: &mut [u8],
    curr_pos: (i32, i32),
    direction: (i32, i32),
    grid_width: i32,
    grid_height: i32,
) {
    let curr_cell = grid[convert_offset(curr_pos.0, curr_pos.1, grid_width)];

    let next_x = curr_pos.0 + direction.0;
    let next_y = curr_pos.1 + direction.1;

    // check if we are in bounds
    if next_x <= 0 || next_x >= grid_width || next_y <= 0 || next_y >= grid_height {
        return;
    }
    let next_c = grid[convert_offset(next_x, next_y, grid_width)];

    // if the next spot is empty. place the current cell in it.
    if next_c == b'.' {
        grid[convert_offset(next_x, next_y, grid_width)] = curr_cell;
        return;
    }

    if next_c == b'O' {
        do_move(grid, (next_x, next_y), direction, grid_width, grid_height);

        grid[convert_offset(next_x, next_y, grid_width)] = curr_cell;
        return;
    }

    if direction.1 != 0 {
        // now the hard part. move the cell vertically
        if next_c == b'[' {
            do_move(grid, (next_x, next_y), direction, grid_width, grid_height);
            do_move(
                grid,
                (next_x + 1, next_y),
                direction,
                grid_width,
                grid_height,
            );

            grid[convert_offset(next_x, next_y, grid_width)] = curr_cell;
            grid[convert_offset(next_x + 1, next_y, grid_width)] = b'.';
        } else if next_c == b']' {
            do_move(grid, (next_x, next_y), direction, grid_width, grid_height);
            do_move(
                grid,
                (next_x - 1, next_y),
                direction,
                grid_width,
                grid_height,
            );

            grid[convert_offset(next_x, next_y, grid_width)] = curr_cell;
            grid[convert_offset(next_x - 1, next_y, grid_width)] = b'.';
        }
    } else {
        // moving horizontal. this is the same as with the small boxes
        do_move(grid, (next_x, next_y), direction, grid_width, grid_height);

        grid[convert_offset(next_x, next_y, grid_width)] = curr_cell;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&mut advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
