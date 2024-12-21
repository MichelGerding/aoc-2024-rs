use rustc_hash::FxHashMap;

use memoize::memoize;
use rayon::prelude::*;

advent_of_code::solution!(21);
const NUMPAD: [(i32, i32); 255] = {
    let mut a = [(0,0); 255];
    a[b'7' as usize] = (0, 0);
    a[b'8' as usize] = (0, 1);
    a[b'9' as usize] = (0, 2);
    a[b'4' as usize] = (1, 0);
    a[b'5' as usize] = (1, 1);
    a[b'6' as usize] = (1, 2);
    a[b'1' as usize] = (2, 0);
    a[b'2' as usize] = (2, 1);
    a[b'3' as usize] = (2, 2);
    a[b'0' as usize] = (3, 1);
    a[b'A' as usize] = (3, 2);
    a
};

const ARROWPAD: [(i32, i32); 255] = {
    let mut a = [(0,0); 255];
    a[b'^' as usize] = (0, 1);
    a[b'A' as usize] = (0, 2);
    a[b'<' as usize] = (1, 0);
    a[b'v' as usize] = (1, 1);
    a[b'>' as usize] = (1, 2);
    a
};

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn press_arrows(i: i32, j: i32, steps: usize, h_first: bool) -> usize {
    if steps == 0 {
        return (i.unsigned_abs() + j.unsigned_abs()) as usize + 1;
    }

    let (ii, jj) = (i.unsigned_abs() as usize, j.unsigned_abs() as usize);
    let mut chunk = Vec::with_capacity(ii + jj + 1);

    if h_first {
        chunk.extend(vec![if j > 0 { '<' } else { '>' }; jj]);
        chunk.extend(vec![if i > 0 { '^' } else { 'v' }; ii]);
    } else {
        chunk.extend(vec![if i > 0 { '^' } else { 'v' }; ii]);
        chunk.extend(vec![if j > 0 { '<' } else { '>' }; jj]);
    }

    chunk.push('A');

    let mut loc = ARROWPAD[b'A' as usize];

    chunk
        .into_iter()
        .map(|c| {
            let n = ARROWPAD[c as u8 as usize];
            let p = loc;
            loc = n;
            let d = (p.0 - n.0, p.1 - n.1);
            if d.0 == 0 || d.1 == 0 {
                // straight line, search only once, order is irrelevant
                press_arrows(d.0, d.1, steps - 1, false)
            } else if n == (1, 0) && p.0 == 0 {
                // must search down first
                press_arrows(d.0, d.1, steps - 1, false)
            } else if p == (1, 0) && n.0 == 0 {
                // must search horiz first
                press_arrows(d.0, d.1, steps - 1, true)
            } else {
                // can search in either order
                std::cmp::min(
                    press_arrows(d.0, d.1, steps - 1, false),
                    press_arrows(d.0, d.1, steps - 1, true),
                )
            }
        })
        .sum()
}

fn press_buttons(sequence: &str, steps: usize) -> usize {
    let mut loc = NUMPAD[b'A' as usize];

    sequence[0..3].parse::<usize>().unwrap()
        * sequence
        .chars()
        .map(|c| {
            // either move horizontally or vertically first
            // in some cases only one will be valid...
            let n = NUMPAD[c as u8 as usize];
            let p = loc;
            let d = (loc.0 - n.0, loc.1 - n.1);
            loc = n;
            if p.0 == 3 && n.1 == 0 {
                // must move up first
                press_arrows(d.0, d.1, steps, false)
            } else if p.1 == 0 && n.0 == 3 {
                // must move right first
                press_arrows(d.0, d.1, steps, true)
            } else {
                // move in either direction
                std::cmp::min(
                    press_arrows(d.0, d.1, steps, true),
                    press_arrows(d.0, d.1, steps, false),
                )
            }
        })
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    memoized_flush_press_arrows();

    Some(input.par_lines().map(|s| press_buttons(s, 2)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    memoized_flush_press_arrows();

    Some(input.par_lines().map(|s| press_buttons(s, 25)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
