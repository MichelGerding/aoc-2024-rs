#![feature(ascii_char)]

use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

const PAGES_COUNT: usize = 199;
// const PAGES_COUNT: usize = 6; // test input
//
const RULES_COUNT: usize = 1176;
// const RULES_COUNT: usize = 21; // test input

fn resolve_page_order(rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut indegree: HashMap<usize, usize> = HashMap::new();

    // Build the graph and calculate indegrees
    for &(a, b) in rules {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default(); // Ensure b is in the graph even if it has no outgoing edges
        *indegree.entry(b).or_default() += 1;
        indegree.entry(a).or_default(); // Ensure a has an indegree entry even if it's 0
    }
    // Collect all nodes with no incoming edges
    let mut queue: VecDeque<usize> = indegree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    let mut result = Vec::new();

    // Perform topological sort
    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = indegree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }


    // If all nodes are processed, return the result; otherwise, there's a cycle
    if result.len() == graph.len() {
        result
    } else {
        panic!("Rules contain a cycle, no valid order exists");
    }
}


fn parse_input(input: &str) -> ([(usize, usize); RULES_COUNT], [Vec<usize>; PAGES_COUNT]) {
    let mut rules = [(0usize, 0usize); RULES_COUNT];
    let mut pages = [const { Vec::new() }; PAGES_COUNT];

    let bytes = input.as_bytes();
    let mut i = 0;
    let mut idx = 0;

    while idx < RULES_COUNT {
        let mut a = 0;
        for c in &bytes[i..i+2] {
            a = a * 10 + (c - b'0') as usize;
        }

        let mut b = 0;
        for c in &bytes[i+3..i+5] {
            b = b * 10 + (c - b'0') as usize;
        }
        rules[idx] = (a, b);

        i += 6;
        idx += 1;
    }

    i += 1;

    let mut idx = 0;
    while idx < PAGES_COUNT {
        let mut j = i;
        while bytes[i] != b'\n' {i += 1}

        while j < i {
            let mut b = 0;
            for c in &bytes[j..j+2] {
                b = b * 10 + (c - b'0') as usize;
            }
            pages[idx].push(b);
            j += 3;
        }

        idx += 1;
        i += 1;
    }

    (rules, pages)
}

fn rules_correct(order_rules: &[(usize, usize)], page_numbers: &[usize]) -> bool {
    order_rules.iter().all(|(a, b)| {
        let a_idx = page_numbers.iter().position(|x| x == a);
        let b_idx = page_numbers.iter().position(|x| x == b);

        match (a_idx, b_idx) {
            (Some(a), Some(b)) => a < b,
            _ => true,
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order_rules, updates) = parse_input(input);

    Some(
        updates.iter().filter(|update| {
            rules_correct(&order_rules, &update)
        }).map(|pages| {
            let len = pages.len();
            pages[len / 2] as u32
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order_rules, updates) = parse_input(input);

    let pages_out_of_order = updates.iter().filter(|update| {
        // check all rules
        !rules_correct(&order_rules, &update)
    }).collect::<Vec<_>>();

    // fix the order of the pages
    let reordered_pages = pages_out_of_order.iter()
        .map(|pages| {
            // get the applying rules
            let rules: Vec<(usize, usize)> = order_rules.iter().filter(|(a, b)| {
                let a_idx = pages.iter().position(|x| x == a);
                let b_idx = pages.iter().position(|x| x == b);
                a_idx.is_some() && b_idx.is_some()
            }).map(|a| *a).collect::<Vec<_>>();

            let page_order = resolve_page_order(&rules);

            let mut page_with_idx = pages.iter().map(|page_nr| {
                let page_order_idx = page_order.iter().position(|x| x == page_nr).unwrap();
                (page_order_idx, *page_nr)
            }).collect::<Vec<(usize, usize)>>();

            page_with_idx.sort_by(|(a, _), (b, _)| a.cmp(b));

            page_with_idx.iter().map(|(_, page_nr)| *page_nr).collect::<Vec<usize>>()
        }).collect::<Vec<_>>();


    let center_pages = reordered_pages
        .iter()
        .map(|pages| {
            let len = pages.len();
            pages[len / 2] as u32
        }).collect::<Vec<u32>>();

    Some(center_pages.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5180));
    }
}
