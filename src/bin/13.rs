advent_of_code::solution!(13);

struct Puzzle {
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    target: (i64, i64),
}


fn solve_button_presses(puzzle: &Puzzle) -> Option<(i64, i64)> {
    let (a_x, a_y) = puzzle.btn_a;
    let (b_x, b_y) = puzzle.btn_b;
    let (target_x, target_y) = puzzle.target;

    let det = a_x * b_y - b_x * a_y;

    let det_a = target_x * b_y - b_x * target_y;
    let det_b = a_x * target_y - target_x * a_y;

    if det_a % det != 0 || det_b % det != 0 {
        return None; // No integer solution
    }

    let a = det_a / det;
    let b = det_b / det;

    Some((a, b))
}


fn parse_target(bytes: &[u8]) -> ((i64, i64), usize) {
    // Locate ',' to determine the end of the X value
    let x_end = bytes[9..].iter().position(|&c| c == b',').unwrap() + 9;
    let end_y = bytes[x_end..].iter().position(|&c| c == b'\n').unwrap() + x_end;
    // Parse X directly from bytes

    let mut x = 0;
    for &b in &bytes[9..x_end] {
        x = x * 10 + (b - b'0') as i64;
    }

    // Parse Y directly from bytes
    let mut y = 0;
    for &b in &bytes[x_end + 4..end_y] {
        y = y * 10 + (b - b'0') as i64;
    }

    ((x, y), end_y)
}

pub fn parse_button(input: &[u8]) -> (i64, i64) {
    let a = ((input[12] - b'0') * 10 + (input[13] - b'0')) as i64;
    let b = ((input[18] - b'0') * 10 + (input[19] - b'0')) as i64;

    (a, b)
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut sum = 0;

    let bytes = input.as_bytes();
    let mut idx = 0;

    while idx <= bytes.len() {
        let btn_a = parse_button(&bytes[idx..=idx + 20]);
        let btn_b = parse_button(&bytes[idx + 21..=idx + 41]);
        let (target, n_idx) = parse_target(&bytes[idx + 42..]);

        idx = idx + 42 + n_idx + 2;

        if let Some((a, b)) = solve_button_presses(&Puzzle {
            btn_a,
            btn_b,
            target,
        }) {
            sum += a * 3 + b;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sum = 0;

    let bytes = input.as_bytes();
    let mut idx = 0;

    while idx <= bytes.len() {
        let btn_a = parse_button(&bytes[idx..=idx + 20]);
        let btn_b = parse_button(&bytes[idx + 21..=idx + 41]);
        let (target, n_idx) = parse_target(&bytes[idx + 42..]);
        let target = (target.0 + 10000000000000, target.1 + 10000000000000);

        idx = idx + 42 + n_idx + 2;

        if let Some((a, b)) = solve_button_presses(&Puzzle {
            btn_a,
            btn_b,
            target,
        }) {
            sum += a * 3 + b;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
