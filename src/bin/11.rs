advent_of_code::solution!(11);
use petgraph::graphmap::GraphMap;
use petgraph::{Directed,Direction};
use std::collections::HashMap;

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
                // println!("adding link from {} to {}", first, other);
            }
        }
    }
    graph
}

fn count_paths<'a>(
    graph: &'a GraphMap::<&'a str, (), Directed>,
    current: &'a str,
    end: &'a str,
    cache: &mut HashMap<&'a str, usize>
) -> usize {
    // Base case: reached the end
    if current == end {
        return 1;
    }

    // Check cache
    if let Some(&count) = cache.get(current) {
        return count;
    }

    // Sum paths through all neighbors
    let total: usize = graph
        .neighbors_directed(current, Direction::Outgoing)
        .map(|neighbor| count_paths(graph, neighbor, end, cache))
        .sum();

    cache.insert(current, total);
    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_input(input);
    let mut cache = HashMap::new();
    let total_paths = count_paths(&graph, "you", "out", &mut cache);
    Some(total_paths)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    // Count paths from svr to out that pass through both fft and dac
    // This is: paths(svr->fft) * paths(fft->dac) * paths(dac->out)
    // Note: Each count_paths call needs its own cache since they have different end points
    let paths_to_fft = count_paths(&graph, "svr", "fft", &mut HashMap::new());
    let paths_through_dac = count_paths(&graph, "fft", "dac", &mut HashMap::new());
    let paths_to_out = count_paths(&graph, "dac", "out", &mut HashMap::new());

    let total = paths_to_fft * paths_through_dac * paths_to_out;
    Some(total as u64)
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
        let result = part_two(&"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out");
        assert_eq!(result, Some(2));
    }
}
