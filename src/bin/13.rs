#![feature(exact_size_is_empty)]

advent_of_code::solution!(13);

type Inputs = (isize, isize);

#[derive(Debug)]
struct Puzzle {
    btn_a: Inputs,
    btn_b: Inputs,
    target: Inputs,
}

fn parse_button(line: &str, split_char: char) -> Inputs {
    let mut parts = line.rsplitn(2, ':');
    let mut parts = parts.next().unwrap().split(',');

    let x_fac = parts
        .next()
        .unwrap()
        .rsplit(split_char)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let y_fac = parts
        .next()
        .unwrap()
        .rsplit(split_char)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    (x_fac, y_fac)
}

fn solve_button_presses(puzzle: &Puzzle) -> Option<Inputs> {
    let (a_x, a_y) = puzzle.btn_a;
    let (b_x, b_y) = puzzle.btn_b;
    let (target_x, target_y) = puzzle.target;

    // Solve the equations:
    // a_x * a + b_x * b = target_x
    // a_y * a + b_y * b = target_y

    let det = a_x * b_y - b_x * a_y;
    // Cramer's rule to find a and b
    let det_a = target_x * b_y - b_x * target_y; // Determinant for "a"
    let det_b = a_x * target_y - target_x * a_y; // Determinant for "b"

    // Check divisibility to ensure integer solutions
    if det_a % det != 0 || det_b % det != 0 {
        return None; // No integer solution
    }

    let a = det_a / det;
    let b = det_b / det;

    if a >= 0 && b >= 0 {
        Some((a, b))
    } else {
        None // No non-negative solution
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(4)
            .filter_map(|chunk| {
                let btn_a = parse_button(chunk[0], '+');
                let btn_b = parse_button(chunk[1], '+');
                let target = parse_button(chunk[2], '=');

                let res = solve_button_presses(&Puzzle {
                    btn_a,
                    btn_b,
                    target,
                });

                res
            })
            .map(|(a, b)| a * 3 + b)
            .sum::<isize>(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(4)
            .filter_map(|chunk| {
                let btn_a = parse_button(chunk[0], '+');
                let btn_b = parse_button(chunk[1], '+');
                let target = parse_button(chunk[2], '=');
                let target = (target.0 + 10000000000000, target.1 + 10000000000000);

                let res = solve_button_presses(&Puzzle {
                    btn_a,
                    btn_b,
                    target,
                });

                res
            })
            .map(|(a, b)| a * 3 + b)
            .sum::<isize>(),
    )
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
