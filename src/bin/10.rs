use petgraph::prelude::*;
use rayon::prelude::*;

advent_of_code::solution!(10);

#[cfg(not(debug_assertions))]
const GRID_SIZE: usize = 55;

#[cfg(debug_assertions)]
const GRID_SIZE: usize = 8;

fn parse_grid_to_graph(input: &str) -> (Graph<(usize, usize, u8), u8>, Vec<NodeIndex>) {
    // Split input once to avoid repeated allocations
    let lines: Vec<&[u8]> = input
        .lines()
        .map(|line| line.as_bytes())
        .collect();


    let mut graph = Graph::new();
    let mut node_map = [NodeIndex::new(0); GRID_SIZE * GRID_SIZE];
    let mut trail_heads = Vec::with_capacity(300);

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            let value = lines[r][c];
            let node = graph.add_node((r, c, value));
            if value == b'0' {
                trail_heads.push(node);
            }
            node_map[r * GRID_SIZE + c] = node;
        }
    }


    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            let current_value = lines[r][c];
            let current_node = node_map[r * GRID_SIZE + c];

            let directions = [
                r.checked_sub(1).map(|nr| (nr, c)),
                if r + 1 < GRID_SIZE { Some((r + 1, c)) } else { None },
                c.checked_sub(1).map(|nc| (r, nc)),
                if c + 1 < GRID_SIZE { Some((r, c + 1)) } else { None },
            ];

            for dir in directions.iter().filter_map(|d| *d) {
                let (nr, nc) = dir;
                if lines[nr][nc] == current_value + 1 {
                    let neighbor_node = node_map[nr * GRID_SIZE + nc];
                    graph.add_edge(current_node, neighbor_node, 1);
                }
            }
        }
    }
    (graph, trail_heads)
}


pub fn part_one(input: &str) -> Option<u32> {
    let (graph, trail_heads) = parse_grid_to_graph(input);

    Some(
        trail_heads
            .par_iter()
            .map(|&start_node| {

                let mut visited = vec![false; GRID_SIZE * GRID_SIZE];
                let mut stack = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

                stack.push(start_node);
                visited[start_node.index()] = true;

                let mut end_count = 0;

                while let Some(node_idx) = stack.pop() {
                    if graph[node_idx].2 == b'9' {
                        end_count += 1;
                    }

                    for neighbor in graph.neighbors(node_idx) {
                        let neighbor_idx = neighbor.index();
                        if !visited[neighbor_idx] {
                            visited[neighbor_idx] = true;
                            stack.push(neighbor);
                        }
                    }
                }

                end_count
            })
            .sum()
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    let (graph, trail_heads) = parse_grid_to_graph(input);

    let sum = trail_heads
        .par_iter()
        .map(|&start_node| {
            let mut stack = vec![(start_node, vec![false; GRID_SIZE * GRID_SIZE])];
            let mut unique_paths = 0;

            while let Some((node, mut visited)) = stack.pop() {
                if graph[node].2 == b'9' {
                    unique_paths += 1;
                }

                visited[node.index()]= true; // Mark the current node as visited

                // Directly mutate the visited set and push onto stack
                for neighbor in graph.neighbors_directed(node, Outgoing) {
                    if !visited[neighbor.index()] {
                        visited[neighbor.index()] = true; // Mark neighbor as visited
                        stack.push((neighbor, visited.clone())); // Push new state (visited state)
                        visited[neighbor.index()] = false; // Unmark the neighbor for next traversal
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
