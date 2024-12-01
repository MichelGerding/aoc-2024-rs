advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    const SIZE: usize = 1000;
    let mut left = [0i32; SIZE];
    let mut right = [0i32; SIZE];

    const NUM_SIZE: usize = 4;
    unsafe {
        let bytes = input.as_bytes();
        let mut i = 0;
        for j in 0..SIZE {
            // Parse first number
            let mut l = 0i32;
            for c in bytes.get_unchecked(i..=i + NUM_SIZE) {
                l = l * 10 + *c as i32;
            }

            // Parse second number
            let mut r = 0i32;
            for c in bytes.get_unchecked(i + 4 + NUM_SIZE..=i + 4 + NUM_SIZE + NUM_SIZE) {
                r = r * 10 + *c as i32;
            }

            // Add to vectors
            *left.get_unchecked_mut(j) = l;
            *right.get_unchecked_mut(j) = r;
            // Advance to next row
            i += 4 + NUM_SIZE + NUM_SIZE + 2;
        }

        left.sort_unstable();
        right.sort_unstable();
    }

    let x = left.iter()
            .zip(right.iter())
            .map(|(l, r)| (*l - *r).abs() as u32)
            .sum::<u32>();

    return Some(x as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    const SIZE: usize = 999;
    const MAX_NUM: usize = 99999 + 1;
    const NUM_SIZE: usize = 4;

    let mut left = [0usize; SIZE];
    let mut right = [0u32; MAX_NUM];

    unsafe {
        let bytes = input.as_bytes();
        let mut i = 0;
        for idx in 0..SIZE {
            // Parse first number
            let mut l = 0usize;
            for c in bytes.get_unchecked(i..=i + NUM_SIZE) {
                l = l * 10 + *c as usize;
            }

            // Parse second number
            let mut r = 0u32;
            for &c in bytes.get_unchecked(i + 4 + NUM_SIZE..=i + 4 + NUM_SIZE + NUM_SIZE) {
                r = r * 10 + c as u32;
            }
            r -= 533328;

            // Add to vectors and HashMap
            *left.get_unchecked_mut(idx) = l;
            *right.get_unchecked_mut(r as usize) += r;


            // Advance to next row
            i += 4 + NUM_SIZE + NUM_SIZE + 2;
        }

        Some(
            left.iter()
                .map(|v| *right.get_unchecked(*v - 533328))
                .sum::<u32>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1222801u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(22545250u32));
    }
}
