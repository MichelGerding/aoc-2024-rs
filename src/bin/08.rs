advent_of_code::solution!(8);

fn parse_input(input: &str) -> ([Vec<(i32, i32)>; 255], i32) {
    let mut antennas: [Vec<(i32, i32)>; 255] = [const { Vec::new() }; 255];
    let mut grid_size: Option<i32> = None;

    for (y, line) in input.lines().enumerate() {
        grid_size = Some(line.len() as i32);
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                antennas[c as usize].push((x as i32, y as i32));
            }
        }
    }

    (antennas, grid_size.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas, grid_size) = parse_input(input);
    let mut antinodes: [u64; 50] = [0u64; 50];

    for antennas in antennas {
        for &a in &antennas {
            for &b in &antennas {
                if a != b {
                    let (ox, oy) = (a.0 - b.0, a.1 - b.1);
                    let (x, y) = (a.0 + ox, a.1 + oy);

                    if x < grid_size && y < grid_size && x >= 0 && y >= 0 {
                        antinodes[y as usize] |= 1 << x;
                    }
                }
            }
        }
    }

    Some(antinodes.iter().map(|v| v.count_ones()).sum())
}


pub fn part_two(input: &str) -> Option<u32> {
    let (antennas, grid_size) = parse_input(input);
    let mut antinodes: [u64; 50] = [0u64; 50];

    for antennas in antennas.iter().filter(|a| !a.is_empty()) {
        for i in 0..antennas.len() {
            let (x1, y1) = antennas[i];

            for &(x2, y2) in &antennas[i + 1..] {
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Closure for extending the line
                let mut draw_line = |mut wx: i32, mut wy: i32, dx: i32, dy: i32| {
                    while wx >= 0 && wy >= 0 && wx < grid_size && wy < grid_size {
                        antinodes[wy as usize] |= 1 << wx;
                        wx += dx;
                        wy += dy;
                    }
                };

                // Draw in both directions
                draw_line(x1, y1, dx, dy);
                draw_line(x1 - dx, y1 - dy, -dx, -dy);
            }
        }
    }
    Some(antinodes.iter().map(|v| v.count_ones()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
