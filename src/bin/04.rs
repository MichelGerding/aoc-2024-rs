advent_of_code::solution!(4);

const GRID_SIZE: i32 = 140;
const UGRID_SIZE: usize = GRID_SIZE as usize;

fn convert_input_to_cells(input: &str) -> [u8; UGRID_SIZE * UGRID_SIZE] {
    unsafe {
        let mut cells = [0u8; UGRID_SIZE * UGRID_SIZE];
        let bytes = input.as_bytes();

        let mut idx = 0;
        let mut i = 0;
        while i < bytes.len() {
            // Avoid branch prediction penalties by using a mask
            let is_not_newline = (bytes.get_unchecked(i) != &b'\n') as u8;
            *cells.get_unchecked_mut(idx) = *bytes.get_unchecked(i);
            idx += is_not_newline as usize;
            i += 1;
        }
        cells
    }
}

fn get_chars_at_offsets(x: i32, y: i32, offsets: &[(i32, i32)], cells: &[u8; UGRID_SIZE * UGRID_SIZE], chars: &mut [u8]) {
    unsafe {
        offsets.iter().enumerate().for_each(|(j, &(dx, dy))| {
            let xo = x + dx;
            let yo = y + dy;

            // Perform bounds check manually
            chars[j] = if xo >= 0 && xo < GRID_SIZE && yo >= 0 && yo < GRID_SIZE {
                *cells.get_unchecked((xo * GRID_SIZE + yo) as usize)
            } else {
                0 // Default value for out-of-bounds
            };
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cells = convert_input_to_cells(input);

    const OFFSETS_GROUPS: [[(i32, i32); 4]; 4] = [
        [(-2, 0), (-1, 0), (0, 0), (1, 0)],
        [(0, 2), (0, 1), (0, 0), (0, -1)],
        [(-2, 2), (-1, 1), (0, 0), (1, -1)],
        [(2, 2), (1, 1), (0, 0), (-1, -1)],
    ];

    let mut xmasses = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            for offsets in OFFSETS_GROUPS.iter() {
                let mut chars = [0u8; 4];
                get_chars_at_offsets(x, y, offsets, &cells, &mut chars);

                if chars == [b'X', b'M', b'A', b'S'] || chars == [b'S', b'A', b'M', b'X'] {
                    xmasses += 1;
                }
            }
        }
    }

    Some(xmasses as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    let cells = convert_input_to_cells(input);

    const OFFSETS_GROUPS: [[(i32, i32); 3]; 2] = [
        [(-1, 1), (0, 0), (1, -1)],
        [(1, 1), (0, 0), (-1, -1)]
    ];

    let mut xmasses = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if OFFSETS_GROUPS.iter().filter(|offsets| {
                let mut chars = [0u8; 3];
                get_chars_at_offsets(x, y, *offsets, &cells, &mut chars);

                chars == [b'M', b'A', b'S'] || chars == [b'S', b'A', b'M']

            }).count() == 2 {
                xmasses += 1;
            }
        }
    }

    Some(xmasses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2551));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1985));
    }
}
