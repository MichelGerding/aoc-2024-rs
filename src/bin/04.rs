advent_of_code::solution!(4);

const GRID_SIZE: usize = 140;

fn get_cell(x: i32, y: i32, cells: &[u8]) -> u8 {
    if (x < 0 || x > (GRID_SIZE - 1) as i32) || (y < 0 || y > (GRID_SIZE - 1) as i32) {
        return 0;
    }

    cells[(x + (y * GRID_SIZE as i32)) as usize]
}

fn convert_input_to_cells(input: &str) -> [u8; GRID_SIZE * GRID_SIZE] {
    let mut cells = [0u8; GRID_SIZE * GRID_SIZE];
    let bytes = input.as_bytes();

    let mut idx = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'\n' {
            cells[idx] = bytes[i];
            idx += 1;
        }

        i += 1
    }

    cells
}


pub fn part_one(input: &str) -> Option<u32> {
    let cells = convert_input_to_cells(input);

    let offsets_groups = [
        [(-2, 0,), (-1, 0,), (0, 0,), (1, 0,)],
        [(0, 2,), (0, 1,), (0, 0,), (0, -1,)],
        [(-2, 2,), (-1, 1,), (0, 0,), (1, -1,)],
        [(2, 2,), (1, 1,), (0, 0,), (-1, -1,)],
    ];

    let mut xmasses = 0;

    for i in 0..cells.len() {
        let x = i / GRID_SIZE;
        let y = i % GRID_SIZE;

        xmasses += offsets_groups.iter().filter(|offsets| {
            let chars = offsets.map(|(dx, dy)| {
                let xo = x as i32 + dx;
                let yo = y as i32 + dy;

                get_cell(xo, yo, &cells)
            });

            chars == [b'X', b'M', b'A', b'S'] || chars == [b'S', b'A', b'M', b'X']
        }).count();
    }


    Some(xmasses as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    let cells = convert_input_to_cells(input);

    let offsets_groups = [
        [(-1, 1,), (0, 0,), (1, -1,)],
        [(1, 1,), (0, 0,), (-1, -1,)],
    ];

    let mut xmasses = 0;

    for i in 0..cells.len() {
        let x = i / GRID_SIZE;
        let y = i % GRID_SIZE;

        if offsets_groups.iter().filter(|offsets| {
            let chars = offsets.map(|(dx, dy)| {
                let xo = x as i32 + dx;
                let yo = y as i32 + dy;

                get_cell(xo, yo, &cells)
            });

            chars == [b'M', b'A', b'S'] || chars == [b'S', b'A', b'M']
        }).count() == 2 {
            xmasses += 1;
        };
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
