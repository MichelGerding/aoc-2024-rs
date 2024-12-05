use rayon::prelude::*;

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


pub fn part_one(input: &str) -> Option<u32> {
    let (order_rules, updates) = parse_input(input);

    Some(
        updates
            .par_iter()
            .filter_map(|update| {
                if !rules_correct(&order_rules, &update) {
                    return None;
                }

                let center = update.len() / 2;
                Some(update[center] as u32)
            }).sum()
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    let (order_rules, mut updates) = parse_input(input);
    Some(
        updates
            .par_iter_mut()
            .filter_map(|pages| {
                unsafe {
                    // check all rules
                    if rules_correct(&order_rules, &pages) {
                        return None;
                    }
                    // get the applying rules and get there order
                    let rules: Vec<(usize, usize)> = applying_rules(&order_rules, &pages);
                    let page_order = resolve_page_order(&rules);

                    // sort based on the rules
                    pages.sort_unstable_by(|a, b| {
                        let ai = page_order.get_unchecked(*a);
                        let bi = page_order.get_unchecked(*b);

                        ai.cmp(bi)
                    });

                    let center = pages.len() / 2;
                    Some(*pages.get_unchecked_mut(center) as u32)
                }
            }).sum())
}

fn parse_input(input: &str) -> ([(usize, usize); RULES_COUNT], [Vec<usize>; PAGES_COUNT]) {
    let bytes = input.as_bytes();
    let mut rules = [(0usize, 0usize); RULES_COUNT];
    let mut pages = [const { Vec::new() }; PAGES_COUNT];

    // Faster parsing using unchecked indexing and direct byte conversion
    unsafe {
        let mut i = 0;
        for idx in 0..RULES_COUNT {
            // Direct fast byte to number conversion
            let a = ((bytes.get_unchecked(i) - b'0') * 10 + (bytes.get_unchecked(i + 1) - b'0')) as usize;
            let b = ((bytes.get_unchecked(i + 3) - b'0') * 10 + (bytes.get_unchecked(i + 4) - b'0')) as usize;

            *rules.get_unchecked_mut(idx) = (a, b);
            i += 6;
        }

        i += 1;

        for idx in 0..PAGES_COUNT {
            let page_vec = pages.get_unchecked_mut(idx);

            loop {
                let b = ((bytes.get_unchecked(i) - b'0') * 10 + (bytes.get_unchecked(i + 1) - b'0')) as usize;
                page_vec.push(b);
                i += 3;
                if *bytes.get_unchecked(i - 1) == b'\n' {
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

unsafe fn resolve_page_order(rules: &[(usize, usize)]) -> [isize; MAX_VALUE + 1] {
    let mut graph = [const { Vec::new() }; MAX_VALUE + 1];
    let mut indegree = [0; MAX_VALUE + 1];

    // Build graph
    for &(a, b) in rules {
        graph.get_unchecked_mut(a).push(b);
        *indegree.get_unchecked_mut(b) += 1;
    }

    // Queue of nodes with zero indegree
    let mut queue: Vec<usize> = indegree.iter().enumerate().filter_map(|(node, &deg)| if deg == 0 { Some(node) } else { None }).collect();

    let mut result = [-1; MAX_VALUE + 1];

    let mut idx = 0;
    // Topological sort
    while let Some(node) = queue.pop() {
        *result.get_unchecked_mut(node) = idx;
        idx += 1;
        for &neighbor in &graph[node] {
            *indegree.get_unchecked_mut(neighbor) -= 1;
            if indegree.get_unchecked(neighbor) == &0 {
                queue.push(neighbor);
            }
        }
    }

    result
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
