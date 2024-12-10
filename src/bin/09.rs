#![feature(let_chains)]

advent_of_code::solution!(9);

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Option<u64>> {
    input.chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, chunk)| {
            let sizes = chunk.collect_vec();
            let file_blocks = sizes[0].to_digit(10).unwrap() as usize;
            let free_blocks = if sizes.len() > 1 {
                sizes[1].to_digit(10).unwrap() as usize
            } else {
                0
            };
            std::iter::repeat_n(Some(id as u64), file_blocks).chain(std::iter::repeat_n(None, free_blocks))
        })
        .flatten()
        .collect_vec()
}

fn calc_checksum(input: &[Option<u64>]) -> Option<u64> {
    Some(
        input
            .iter()
            .enumerate()
            .map(|(idx, o)| match o {
                None => 0,
                Some(v) => (idx as u64) * *v,
            })
            .sum()
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map = parse_input(input);

    let mut l = 0usize;
    let mut r = disk_map.iter().rposition(|o| o.is_some()).unwrap();
    while l < r {
        if disk_map[l].is_none() {
            disk_map.swap(l, r);
            while disk_map[r].is_none() {
                r -= 1;
            }
        }
        l += 1;
    }

    calc_checksum(&disk_map)
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_map = parse_input(input);

    let mut files_try_moved: HashSet<u64> = HashSet::new();
    let mut free_spaces = get_all_free_spaces(&disk_map);

    let mut file = None;
    loop {
        file = find_next_file(&disk_map, file.map_or(disk_map.len() - 1, |(l, _)| l - 1));
        match file {
            None => break,
            Some(file) => {
                let id = disk_map[file.0].unwrap();
                if !files_try_moved.insert(id) {
                    continue;
                }

                let filesize = file.1 - file.0;

                if let Some((free_space_size, free_space_start)) = free_spaces
                    .iter()
                    .flat_map(|(size, heap)| heap.peek().map(|s| (*size, s.0)))
                    .filter(|(size, start)| *size >= filesize && *start < file.0)
                    .sorted_by(|a, b| a.1.cmp(&b.1))
                    .next()
                {
                    for i in 0..=filesize {
                        disk_map[free_space_start + i] = disk_map[file.0 + i];
                        disk_map[file.0 + i] = None;
                    }

                    // Remove the one we chose
                    free_spaces.get_mut(&free_space_size).unwrap().pop();

                    if free_space_size > filesize {
                        let new_size = free_space_size - filesize - 1;
                        if let Some(h) = free_spaces.get_mut(&new_size) {
                            h.push(Reverse(free_space_start + filesize + 1));
                        } else {
                            free_spaces.insert(
                                new_size,
                                BinaryHeap::from([Reverse(free_space_start + filesize + 1)]),
                            );
                        }
                    }
                }
            }
        }
    }

    calc_checksum(&disk_map)
}

fn find_next_file(disk_map: &Vec<Option<u64>>, start: usize) -> Option<(usize, usize)> {
    let mut r = start;
    loop {
        match disk_map.get(r) {
            None => return None,
            Some(Some(_)) => break,
            Some(None) => {
                r -= 1;
            }
        }
    }
    let mut l = r;
    loop {
        match disk_map.get(l) {
            None => break,
            Some(Some(v)) => {
                if *v != disk_map[r].unwrap() {
                    break;
                }
                l -= 1;
            }
            Some(None) => break,
        }
    }
    Some((l + 1, r))
}

fn get_all_free_spaces(disk_map: &Vec<Option<u64>>) -> HashMap<usize, BinaryHeap<Reverse<usize>>> {
    let mut free_spaces: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();

    let find_next_free_space = |start: usize| -> Option<(usize, usize)> {
        let mut l = start;
        loop {
            match disk_map.get(l) {
                None => return None,
                Some(Some(_)) => {
                    l += 1;
                }
                Some(None) => break,
            }
        }
        let mut r = l;
        loop {
            match disk_map.get(r) {
                None => break,
                Some(Some(_)) => break,
                Some(None) => {
                    r += 1;
                }
            }
        }
        Some((l, r - 1))
    };

    let mut free_space = None;
    loop {
        free_space = find_next_free_space(free_space.map_or(0, |(_, r)| r + 1));
        match free_space {
            None => break,
            Some(free_space) => {
                let size = free_space.1 - free_space.0;
                if let Some(h) = free_spaces.get_mut(&size) {
                    h.push(Reverse(free_space.0));
                } else {
                    free_spaces.insert(size, BinaryHeap::from([Reverse(free_space.0)]));
                }
            }
        }
    }

    free_spaces
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
