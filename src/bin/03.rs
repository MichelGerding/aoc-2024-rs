use fancy_regex::Regex;
use lazy_static::lazy_static;

advent_of_code::solution!(3);

lazy_static! {
    static ref DO_RE: Regex = Regex::new(r"(?<=^|do\(\))((.|\n)*?)(?=don't\(\)|$)").unwrap();
}

pub fn multsum_captures(bytes: &[u8]) -> u32 {
    let mut sum = 0;

    // let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Look for "mul("
        if i + 4 <= bytes.len() && &bytes[i..i + 4] == b"mul(" {
            i += 4;

            let mut a = 0u32;
            let mut b = 0u32;
            let mut parsing_a = true;

            // Parse numbers until ')'
            while i < bytes.len() {
                match bytes[i] {
                    b')' => {
                        sum += a * b;
                        i += 1;
                        break;
                    }
                    b',' if parsing_a => {
                        parsing_a = false;
                    }
                    b'0'..=b'9' => {
                        let digit = (bytes[i] - b'0') as u32;
                        if parsing_a {
                            a = a * 10 + digit;
                        } else {
                            b = b * 10 + digit;
                        }
                    }
                    _ => {
                        // Invalid character; break out
                        break;
                    }
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(multsum_captures(input.as_bytes()))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut start = 0;
    let bytes = input.as_bytes();

    while start < bytes.len() {
        // Look for "do("
        if start + 3 <= bytes.len() && &bytes[start..start + 3] == b"do(" {
            start += 3;
            let mut end = start;

            // Find the matching "don't()" or end of string
            while end < bytes.len() {
                if end + 6 <= bytes.len() && &bytes[end..end + 6] == b"don't" {
                    break;
                }
                end += 1;
            }

            // Extract and process the substring
            sum += multsum_captures(&bytes[start..end]);
            start = end + 6; // Move past "don't()"
        } else {
            start += 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(175700056));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71668682));
    }
}
