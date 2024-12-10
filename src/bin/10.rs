use ahash::{AHashMap};
use petgraph::prelude::*;
use rayon::prelude::*;


advent_of_code::solution!(10);

fn parse_grid_to_graph(input: &str) -> (Graph<(usize, usize, u8), u8>, Vec<(usize, usize)>) {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut graph = Graph::new();
    let mut node_map = AHashMap::with_capacity(rows * cols);
    let mut trail_heads = Vec::with_capacity(rows * cols);

    // Step 1: Create nodes and store their indices in parallel
    grid.par_iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, &value)| {
            unsafe {
                let node = graph.add_node((r, c, value));
                node_map.insert((r, c), node);
                if value == b'0' {
                    trail_heads.push((r, c));
                }
            }
        });
    });

    // Step 2: Create edges between nodes in parallel
    (0..rows).into_par_iter().for_each(|r| {
        (0..cols).for_each(|c| {
            unsafe {
                let current_value = grid[r][c];
                let current_node = *node_map.get(&(r, c)).unwrap();

                if r > 0 && grid[r - 1][c] == current_value + 1 {
                    let neighbor_node = *node_map.get(&(r - 1, c)).unwrap();
                    graph.add_edge(current_node, neighbor_node, 1);
                }
                if r < rows - 1 && grid[r + 1][c] == current_value + 1 {
                    let neighbor_node = *node_map.get(&(r + 1, c)).unwrap();
                    graph.add_edge(current_node, neighbor_node, 1);
                }
                if c > 0 && grid[r][c - 1] == current_value + 1 {
                    let neighbor_node = *node_map.get(&(r, c - 1)).unwrap();
                    graph.add_edge(current_node, neighbor_node, 1);
                }
                if c < cols - 1 && grid[r][c + 1] == current_value + 1 {
                    let neighbor_node = *node_map.get(&(r, c + 1)).unwrap();
                    graph.add_edge(current_node, neighbor_node, 1);
                }
            }
        });
    });

    (graph, trail_heads)
}



pub fn part_one(input: &str) -> Option<u32> {
    let (graph, trail_heads) = parse_grid_to_graph(input);



    let sum = trail_heads.par_iter().map(|&start| {
        let start_node = graph
            .node_indices()
            .find(|&n| graph[n].0 == start.0 && graph[n].1 == start.1)
            .unwrap();

        let mut end_count = 0;
        let mut dfs = Dfs::new(&graph, start_node);

        while let Some(node) = dfs.next(&graph) {
            let value = graph[node].2;
            if value == b'9' {
                end_count += 1;
            }
        }

        end_count
    }).sum();

    Some(sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let (graph, trail_heads) = parse_grid_to_graph(input);

    let sum = trail_heads.par_iter().map(|&start| {
        let start_node = graph
            .node_indices()
            .find(|&n| graph[n].0 == start.0 && graph[n].1 == start.1)
            .unwrap();

        let mut stack = vec![(start_node, vec![start_node])];
        let mut unique_paths = 0;

        while let Some((node, path)) = stack.pop() {
            let value = graph[node].2;

            if value == b'9' {
                unique_paths += 1; // Found a unique path ending at a 9
                continue;
            }

            for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
                if !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    stack.push((neighbor, new_path));
                }
            }
        }

        unique_paths
    }).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
