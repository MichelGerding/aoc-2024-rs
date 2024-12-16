use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use rustc_hash::FxHashSet;

advent_of_code::solution!(16);

type Coord = (usize, usize);
type PredMap = HashMap<(Coord, usize), Vec<(Coord, usize)>>;

static MOVE_TOWARD: [fn(Coord) -> Coord; 4] = [
    |(x, y)| (x + 1, y),
    |(x, y)| (x, y - 1),
    |(x, y)| (x - 1, y),
    |(x, y)| (x, y + 1),
];


fn move_or_rotate(curr_pos: Coord, curr_dir: usize, rot: usize) -> (Coord, usize) {
    let next_dir = (curr_dir + rot) % 4;
    let next_pos = if rot == 0 {
        MOVE_TOWARD[next_dir](curr_pos)
    } else {
        curr_pos
    };
    (next_pos, next_dir)
}

fn find_path(map: &[Vec<char>]) -> Option<(Coord, usize, isize, PredMap)> {
    #[cfg(test)]
    let (start, goal) = (
        (15, 1), (1,15)
    );

    #[cfg(not(test))]
    let (start, goal) = (
        (139, 1), (1,139)
    );

    let (init_dir, init_score) = (0usize, 0isize);
    let mut stack = VecDeque::from([(start, init_dir, init_score)]);
    let mut path_node = HashMap::from([((start, init_dir), init_score)]);
    let mut preds = PredMap::new();

    while let Some((curr_pos, curr_dir, score)) = stack.pop_front() {
        for (rot, cost) in [(3, -1000), (0, -1), (1, -1000)] {
            let next_node @ (next_pos, next_dir) = move_or_rotate(curr_pos, curr_dir, rot);
            let new_score = score + cost;
            let old_score = path_node.get(&(next_pos, next_dir)).copied();

            if map[next_pos.1][next_pos.0] != '#' && Some(new_score) >= old_score {
                if Some(new_score) > old_score && curr_pos != goal {
                    stack.push_back((next_pos, next_dir, new_score));
                }

                path_node.remove(&next_node);
                path_node.insert(next_node, new_score);

                // Update next node predecessors
                let mut new_preds = vec![(curr_pos, curr_dir)];
                let olds = preds.remove(&next_node).unwrap_or_default().into_iter();
                let keep_old_preds = old_score == Some(new_score);
                new_preds.extend(olds.take_while(|_| keep_old_preds));
                preds.insert(next_node, new_preds);
            }
        }
    }
    let (direction_arrival, score) = (0..4)
        .filter_map(|dir| Some((dir, path_node.get(&(goal, dir))?)))
        .max_by_key(|(_dir, score)| **score)?;
    Some((goal, direction_arrival, -*score, preds))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn part_one(input: &str) -> Option<isize> {
    let map = parse_input(input);
    Some(find_path(&map)?.2)
}

fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);


    let (goal, arrival_dir, _, preds) = find_path(&map)?;

    let mut visited = FxHashSet::default();
    let mut stack_back = VecDeque::from([(goal, arrival_dir)]);
    while let Some((pos, curr_dir)) = stack_back.pop_front() {
        match (visited.insert((pos, curr_dir)), preds.get(&(pos, curr_dir))) {
            (true, Some(preds)) => stack_back.extend(preds),
            _ => (),
        }
    }

    Some(visited.into_iter().counts_by(|(coord, _dir)| coord).len())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
