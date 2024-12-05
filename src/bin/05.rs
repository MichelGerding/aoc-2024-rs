#![feature(ascii_char)]

advent_of_code::solution!(5);

#[cfg(not(debug_assertions))]
const PAGES_COUNT: usize = 199;
#[cfg(debug_assertions)]
const PAGES_COUNT: usize = 6;

#[cfg(not(debug_assertions))]
const RULES_COUNT: usize = 1176;
#[cfg(debug_assertions)]
const RULES_COUNT: usize = 21;

const MAX_VALUE: usize = 99;


fn parse_input(input: &str) -> ([(usize, usize); RULES_COUNT], [Vec<usize>; PAGES_COUNT]) {
    let bytes = input.as_bytes();
    let mut rules = [(0usize, 0usize); RULES_COUNT];
    let mut pages = [const { Vec::new() }; PAGES_COUNT];

    // Faster parsing using unchecked indexing and direct byte conversion
    unsafe {
        let mut i = 0;
        for idx in 0..RULES_COUNT {
            // Direct fast byte to number conversion
            let a = ((bytes.get_unchecked(i) - b'0') * 10 + (bytes.get_unchecked(i+1) - b'0')) as usize;
            let b = ((bytes.get_unchecked(i+3) - b'0') * 10 + (bytes.get_unchecked(i+4) - b'0')) as usize;

            *rules.get_unchecked_mut(idx) = (a, b);
            i += 6;
        }

        i += 1;

        for idx in 0..PAGES_COUNT {
            let page_vec = pages.get_unchecked_mut(idx);

            loop {
                let b = ((bytes.get_unchecked(i) - b'0') * 10 + (bytes.get_unchecked(i+1) - b'0')) as usize;
                page_vec.push(b);
                i += 3;
                if *bytes.get_unchecked(i-1) == b'\n' {
                    break;
                }

            }
        }
    }

    (rules, pages)
}

fn rules_correct(order_rules: &[(usize, usize)], page_numbers: &[usize]) -> bool {
    unsafe {
        // cache the lookup for the index of a number in page_numbers
        let mut p_cache = [0usize; MAX_VALUE + 1];
        for (i, &nr) in page_numbers.iter().enumerate() {
            *p_cache.get_unchecked_mut(nr) = i + 1;
        }

        order_rules.iter().all(|(a, b)| {
            let a: usize = *p_cache.get_unchecked(*a);
            if a == 0 { return true; }

            let b: usize = *p_cache.get_unchecked(*b);
            if b == 0 { return true; }

            return a < b;
        })
    }
}

fn applying_rules(order_rules: &[(usize, usize)], page_numbers: &[usize]) -> Vec<(usize, usize)> {
    unsafe {
        // cache the lookup for the index of a number in page_numbers
        let mut p_cache = [0usize; MAX_VALUE + 1];
        for (i, &nr) in page_numbers.iter().enumerate() {
            *p_cache.get_unchecked_mut(nr) = i + 1;
        }

        // check if the rules apply. a value of 0 indicates it isn't in page_numbers
        order_rules.iter().filter(|(a, b)| {
            let a: usize = *p_cache.get_unchecked(*a);
            if a == 0 { return false; }

            let b: usize = *p_cache.get_unchecked(*b);
            if b == 0 { return false; }
            return true;
        }).map(|a| *a).collect::<Vec<_>>()
    }
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

unsafe fn resolve_page_order(rules: &[(usize, usize)]) -> Vec<usize> {
    let mut graph = [const { Vec::new() }; MAX_VALUE + 1];
    let mut indegree = [0; MAX_VALUE + 1];

    // Build graph
    for &(a, b) in rules {
        graph.get_unchecked_mut(a).push(b);
        *indegree.get_unchecked_mut(b) += 1;
    }

    // Queue of nodes with zero indegree
    let mut queue: Vec<usize> = indegree.iter().enumerate()
        .filter_map(|(node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    let mut result = Vec::with_capacity(PAGES_COUNT);

    // Topological sort
    while let Some(node) = queue.pop() {
        result.push(node);
        for &neighbor in &graph[node] {
            *indegree.get_unchecked_mut(neighbor) -= 1;
            if indegree.get_unchecked(neighbor) == &0 {
                queue.push(neighbor);
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order_rules, updates) = parse_input(input);

    let pages_out_of_order = updates.iter().filter(|update| {
        // check all rules
        !rules_correct(&order_rules, &update)
    }).collect::<Vec<_>>();

    unsafe {
        // fix the order of the pages
        let reordered_pages = pages_out_of_order.iter()
            .map(|pages| {
                // get the applying rules and get there order
                let rules: Vec<(usize, usize)> = applying_rules(&order_rules, &pages);
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
                *pages.get_unchecked(len / 2) as u32
            }).collect::<Vec<u32>>();

        Some(center_pages.iter().sum())
    }
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
