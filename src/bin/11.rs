advent_of_code::solution!(11);
use petgraph::graphmap::GraphMap;
use petgraph::{Directed,Direction};
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> GraphMap::<&str, (), Directed> {
    let mut graph = GraphMap::<&str, (), Directed>::new();

    for line in input.lines() {
        if let Some((first, rest_str)) = line.split_once(':') {
            let first = first.trim();
            let others: Vec<&str> = rest_str.split_whitespace().collect();
            if !graph.contains_node(first) {
                graph.add_node(first);
            }
            for other in others {
                if !graph.contains_node(other) {
                    graph.add_node(other);
                }
                graph.add_edge(first, other, ());
                println!("adding link from {} to {}", first, other);
            }
        }
    }
    graph
}

fn get_pred_from_node<'a>(
    graph: &'a GraphMap::<&'a str, (), Directed>,
    node: &'a str,
    cache: &mut HashMap<&'a str, usize>
) -> usize {
    if let Some(&cached) = cache.get(node) {
        return cached;
    }

    // Find nodes on paths from "you" to target
    let on_path = find_nodes_on_paths(graph, "you", node);

    // Process nodes in topological order to count paths
    let mut in_degree: HashMap<&str, usize> = on_path.iter().map(|&n| (n, 0)).collect();
    for &n in &on_path {
        for pred in graph.neighbors_directed(n, Direction::Incoming) {
            if in_degree.contains_key(pred) {
                *in_degree.get_mut(&n).unwrap() += 1;
            }
        }
    }

    let mut queue: VecDeque<&str> = on_path.iter()
        .filter(|&&n| in_degree[&n] == 0)
        .copied()
        .collect();

    while let Some(current) = queue.pop_front() {
        if cache.contains_key(current) {
            update_successors(graph, current, &mut in_degree, &mut queue);
            continue;
        }

        let path_count: usize = graph.neighbors_directed(current, Direction::Incoming)
            .filter(|&p| on_path.contains(&p))
            .map(|p| cache.get(p).copied().unwrap_or(0))
            .sum();

        println!("node {} has {} paths", current, path_count);
        cache.insert(current, path_count);
        update_successors(graph, current, &mut in_degree, &mut queue);
    }

    cache.get(node).copied().unwrap_or(0)
}

fn find_nodes_on_paths<'a>(
    graph: &'a GraphMap::<&'a str, (), Directed>,
    start: &'a str,
    end: &'a str
) -> HashSet<&'a str> {
    // Forward BFS from start
    let mut reachable_from_start = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    reachable_from_start.insert(start);

    while let Some(current) = queue.pop_front() {
        for succ in graph.neighbors_directed(current, Direction::Outgoing) {
            if reachable_from_start.insert(succ) {
                queue.push_back(succ);
            }
        }
    }

    // Backward BFS from end, keeping only nodes reachable from start
    let mut can_reach_end = HashSet::new();
    queue.push_back(end);
    can_reach_end.insert(end);

    while let Some(current) = queue.pop_front() {
        if current == start {
            continue;  // Don't go beyond start
        }
        for pred in graph.neighbors_directed(current, Direction::Incoming) {
            if reachable_from_start.contains(pred) && can_reach_end.insert(pred) {
                queue.push_back(pred);
            }
        }
    }

    can_reach_end
}

fn update_successors<'a>(
    graph: &'a GraphMap::<&'a str, (), Directed>,
    node: &'a str,
    in_degree: &mut HashMap<&'a str, usize>,
    queue: &mut VecDeque<&'a str>
) {
    for succ in graph.neighbors_directed(node, Direction::Outgoing) {
        if let Some(deg) = in_degree.get_mut(&succ) {
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(succ);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_input(input);
    let mut cache = HashMap::new();
    cache.insert("you", 1);  // Start with 1 path at "you"
    let total_paths = get_pred_from_node(&graph, "out", &mut cache);
    Some(total_paths)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
