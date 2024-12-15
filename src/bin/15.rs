use advent_of_code::convert_offset;

advent_of_code::solution!(15);

macro_rules! generate_translations {
    () => {{
        const fn generate() -> [(i32, i32); 255] {
            let mut m = [(0, 0); 255];
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

    let mut bytes = input.as_bytes().to_owned();

    let moves = unsafe {
        std::slice::from_raw_parts(
            input
                .as_ptr()
                .add((GRID_SIZE * GRID_SIZE + GRID_SIZE + 1) as usize),
            input.len() - (GRID_SIZE * GRID_SIZE + GRID_SIZE + 1) as usize,
        )
    };

    let mut current_pos: (i32, i32) = (
        GRID_SIZE as i32 / 2 - 1,
        GRID_SIZE as i32 / 2 - 1, // 2,2
    );

    bytes[convert_offset(current_pos.0, current_pos.1, GRID_SIZE)] = b'.';

    let mut moves_idx = 0;
    while moves_idx < moves.len() {
        let m = moves[moves_idx];
        if m == b'\n' {
            moves_idx += 1;
            continue;
        }

        let t = TRANSLATIONS[m as usize];

        moves_idx += 1;

        if can_move(&mut bytes, current_pos, t, GRID_SIZE, GRID_SIZE) {
            do_move(&mut bytes, current_pos, t, GRID_SIZE, GRID_SIZE);
            current_pos = (current_pos.0 + t.0, current_pos.1 + t.1);
        } else {
            if moves[moves_idx] == m {
                moves_idx += 1;
            }
        }
    }

    Some(score_grid(&bytes, GRID_SIZE))
}

pub fn transform_input(input: &str) -> (Vec<u8>, &str) {
    let mut bytes = Vec::with_capacity(input.len() * 2);
    let grid_len = 50 * 50 + 50;

    for (idx, ch) in input.chars().enumerate() {
        if idx >= grid_len {
            break;
        }
        match ch {
            '#' => bytes.extend_from_slice(b"##"),
            'O' => bytes.extend_from_slice(b"[]"),
            '.' => bytes.extend_from_slice(b".."),
            '@' => bytes.extend_from_slice(b".."),
            _ => bytes.extend_from_slice(ch.to_string().as_bytes()),
        }
    }

    (bytes, &input[grid_len + 1..])
}

pub fn part_two(input: &str) -> Option<i32> {
    #[cfg(test)]
    const GRID_HEIGHT: i32 = 10;

    #[cfg(not(test))]
    const GRID_HEIGHT: i32 = 50;
    const GRID_WIDTH: i32 = GRID_HEIGHT * 2;

    // transform the input
    let (bytes, moves) = &mut transform_input(input);
    let moves = moves.as_bytes();

    // initial position of the robot. it is slightly offset from the center due to the transformation
    // this moves it 1 to the left. The indexing is also done from 0 so we always need to subtract one
    let mut current_pos = (GRID_WIDTH / 2 - 2, GRID_HEIGHT / 2 - 1);

    // the moves start after the grid.
    let mut moves_idx = 0;
    while moves_idx < moves.len() {
        // for _ in 0..1 {
        let m = moves[moves_idx];
        moves_idx += 1;

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
            if moves[moves_idx] == m {
                moves_idx += 1;
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
    (curr_x, curr_y): (i32, i32),
    (dx, dy): (i32, i32),
    grid_width: i32,
    grid_height: i32,
) -> bool {
    let next_x = curr_x + dx;
    let next_y = curr_y + dy;

    // Check if the next position is out of bounds (without bounds checking in each step)
    if next_x <= 0 || next_y <= 0 || next_x > grid_width || next_y > grid_height {
        return false;
    }

    let next_c = unsafe { *grid.get_unchecked(convert_offset(next_x, next_y, grid_width)) };

    match (next_c, dx) {
        (b'#', _) => false, // Wall, can't move
        (b'.', _) => true,  // Empty space, can move
        (b'[', 0) | (b']', 0) => {
            let offset_x = next_x + (b'\\' as i32 - next_c as i32);
            // Combine checks for the current position and the neighboring bracket
            can_move(grid, (next_x, next_y), (dx, dy), grid_width, grid_height)
                && can_move(grid, (offset_x, next_y), (dx, dy), grid_width, grid_height)
        }
        // it is not a wall so it should be a small box or a horizontal movement
        _ => can_move(grid, (next_x, next_y), (dx, dy), grid_width, grid_height),
    }
}

fn do_move(
    grid: &mut [u8],
    (curr_x, curr_y): (i32, i32),
    (dx, dy): (i32, i32),
    grid_width: i32,
    grid_height: i32,
) {
    let next_x = curr_x + dx;
    let next_y = curr_y + dy;

    // check if we are in bounds
    if next_x <= 0 || next_x >= grid_width || next_y <= 0 || next_y >= grid_height {
        return;
    }

    let next_offset = convert_offset(next_x, next_y, grid_width);
    let curr_offset = convert_offset(curr_x, curr_y, grid_width);

    let next_c = grid[next_offset];
    match (next_c, dx) {
        (b'.', _) => {}
        (b'[', 0) | (b']', 0) => {
            // only use the complex movement for pushing wide crates vertically

            // characters '[' and ']' are seperated by '\' on the ascii table. we can subtract
            // the bracket form '\' to get the correct offset. -1 for left 1 for right
            let offset = b'\\' as i32 - next_c as i32;
            let offset_x = next_x + offset;

            do_move(grid, (next_x, next_y), (dx, dy), grid_width, grid_height);
            do_move(grid, (offset_x, next_y), (dx, dy), grid_width, grid_height);

            grid[(next_offset as i32 + offset) as usize] = b'.';
        }
        _ => {
            // move normally for horizontal movement and small crates
            do_move(grid, (next_x, next_y), (dx, dy), grid_width, grid_height);
        }
    }

    grid[convert_offset(next_x, next_y, grid_width)] = grid[curr_offset];
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
