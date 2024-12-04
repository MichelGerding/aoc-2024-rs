use fancy_regex::{Regex};

advent_of_code::solution!(3);

pub fn multsum_captures(input: &str, re: &Regex) -> u32 {
    re.captures_iter(input).map(|caps| {
        // mulitly first and second capture groups
        let caps = caps.unwrap();

        let a = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let b = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        return a * b;
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    Some(multsum_captures(input, &re))
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_re = Regex::new(r"(?<=^|do\(\))((.|\n)*?)(?=don't\(\)|$)").unwrap();
    let mult_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    Some(do_re.captures_iter(input).map(|a| {
        let a = a.as_ref().unwrap().get(1).unwrap().as_str();

        multsum_captures(a, &mult_re)
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71668682));
    }
}
