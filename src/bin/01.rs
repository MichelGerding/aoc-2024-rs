advent_of_code::solution!(1);

#[cfg(not(debug_assertions))]
const SIZE: usize = 1000;
#[cfg(not(debug_assertions))]
const NUM_SIZE: usize = 5;
#[cfg(not(debug_assertions))]
const ASCII_CONVERSION_FACTOR: usize = 533328; // 48 * 11_111: 48 * ('1'*(NUMSIZE+1))

#[cfg(debug_assertions)]
const SIZE: usize = 6;
#[cfg(debug_assertions)]
const NUM_SIZE: usize = 1;
#[cfg(debug_assertions)]
const ASCII_CONVERSION_FACTOR: usize = 48; // 48 * 1: 48 * ('1'*(NUMSIZE+1))

const OFFSET_NEXT_NUM: usize = NUM_SIZE + 3;
const MAX_NUM: usize = 100000;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = [0i32; SIZE];
    let mut right = [0i32; SIZE];

    unsafe {
        let bytes = input.as_bytes();
        let mut i = 0;
        for j in 0..SIZE {
            // Parse first number
            let mut l = 0;
            for c in bytes.get_unchecked(i..i + NUM_SIZE) {
                l = l * 10 + *c as i32;
            }

            // Parse second number
            let mut r = 0;
            for c in bytes.get_unchecked(i + OFFSET_NEXT_NUM..i + OFFSET_NEXT_NUM + NUM_SIZE) {
                r = r * 10 + *c as i32;
            }

            // Add to vectors
            *left.get_unchecked_mut(j) = l;
            *right.get_unchecked_mut(j) = r;
            // Advance to next row
            i += OFFSET_NEXT_NUM + NUM_SIZE + 1;
        }

        left.sort_unstable();
        right.sort_unstable();
    }

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (*l - *r).abs())
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = [0usize; SIZE];
    let mut right = [0usize; MAX_NUM];

    unsafe {
        let bytes = input.as_bytes();
        let mut i = 0;
        for idx in 0..SIZE {
            // Parse first number
            let mut l = 0;
            for c in bytes.get_unchecked(i..i + NUM_SIZE) {
                l = l * 10 + *c as usize;
            }
            l -= ASCII_CONVERSION_FACTOR;

            // Parse second number
            let mut r = 0;
            for c in bytes.get_unchecked(i + OFFSET_NEXT_NUM..i + OFFSET_NEXT_NUM + NUM_SIZE) {
                r = r * 10 + *c as usize;
            }
            r -= ASCII_CONVERSION_FACTOR; // 533328

            // Add to vectors and HashMap
            *left.get_unchecked_mut(idx) = l;
            *right.get_unchecked_mut(r) += r;

            // Advance to next row
            i += OFFSET_NEXT_NUM + NUM_SIZE + 1;
        }

        Some(left.iter().map(|v| *right.get_unchecked(*v)).sum::<usize>() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
