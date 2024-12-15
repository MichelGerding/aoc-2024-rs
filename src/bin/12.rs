use std::collections::VecDeque;

advent_of_code::solution!(12);

struct Region {
    area: i32,
    horizontal_sides: Vec<(i32, i32)>,
    vertical_sides: Vec<(i32, i32)>
}

fn fill_region(
    sx: i32,
    sy: i32,
    grid: &[u8],
    width: usize,
    height: usize,
    seen: &mut [bool],
) -> Region {
    let mut area = 0;
    let mut horizontal_sides = Vec::new();
    let mut vertical_sides = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    seen[sy as usize * width + sx as usize] = true;

    while let Some((x, y)) = queue.pop_front() {
        let c = grid[y as usize * width + x as usize];
        area += 1;

        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + d.0;
            let ny = y + d.1;
            let ni = ny * width as i32 + nx;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 && grid[ni as usize] == c {
                if !seen[ni as usize] {
                    seen[ni as usize] = true;
                    queue.push_back((nx, ny));
                }
            } else if d.1 == 0 {
                vertical_sides.push((y, x * 4 + d.0));
            } else {
                horizontal_sides.push((x, y * 4 + d.1));
            }
        }
    }

    Region {
        area,
        horizontal_sides,
        vertical_sides,
    }
}

fn remove_connected(s: (i32, i32), sides: &mut Vec<(i32, i32)>) {
    // since there's always only a very small number of side tiles, it's
    // faster to use a Vec instead of a HashSet or a BinaryHeap
    let mut a = s.0 + 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a += 1;
    }
    let mut a = s.0 - 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a -= 1;
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut seen = vec![false; grid.len()];
    let mut price = 0;

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if seen[y as usize * width + x as usize] {
                continue;
            }

            let region = fill_region(x, y, &grid, width, height, &mut seen);
            price += region.area * (region.horizontal_sides.len() + region.vertical_sides.len()) as i32;
        }
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut seen = vec![false; grid.len()];
    let mut price = 0;
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if seen[y as usize * width + x as usize] {
                continue;
            }

            // Fill region. This will give us its area and all its horizontal
            // and vertical side tiles.
            let region = fill_region(x, y, &grid, width, height, &mut seen);

            // find connected side tiles and count how many sides there are
            let mut n_sides = 0;
            for sides in [region.horizontal_sides, region.vertical_sides].iter_mut() {
                while !sides.is_empty() {
                    let s = sides.swap_remove(0);
                    remove_connected(s, sides);
                    n_sides += 1;
                }
            }

            price += region.area * n_sides;
        }
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
