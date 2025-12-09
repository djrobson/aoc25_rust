advent_of_code::solution!(8);
use glam::i64::I64Vec3;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<I64Vec3> {
    input.lines().map(|line| {
            let mut nums = line.split(',');
            let x = nums.next().unwrap().parse().unwrap();
            let y = nums.next().unwrap().parse().unwrap();
            let z = nums.next().unwrap().parse().unwrap();
        
        I64Vec3::new(x,y,z)
    }).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_impl(input, 1000)
}

fn part_one_impl(input: &str, max_connections: usize) -> Option<usize> {
    let mut points = parse_input(input);

    // there are only unique X's in the input
    points.sort_by_key(|pt| pt.x);

    let mut distances: HashMap<i64, (usize,usize)> = HashMap::new();
    for pt1_idx in 0..points.len() {
        for pt2_idx in (pt1_idx+1)..points.len() {
            let distance = points[pt1_idx].distance_squared(points[pt2_idx]);

            if distances.contains_key(&distance) {
                println!("found a tied distance {:?}->{:?} = {} = {:?}", 
                    points[pt1_idx], points[pt2_idx], distance, distances.get(&distance).unwrap());
                distances.insert(distance+1,(pt1_idx, pt2_idx));
            } else {
                distances.insert(distance,(pt1_idx, pt2_idx));
            }
        }
    }
    println!("calculated {} distances", distances.len());
    // we haven't seen any point indexes yet
    let mut unseen: HashSet<usize> = HashSet::from_iter(0..points.len());
    let mut seen_groups: Vec<HashSet<usize>> = Vec::new();
    let mut shortest_distances = distances.keys().sorted();
    let mut connections_made = 0;
    while connections_made < max_connections {
        let distance = shortest_distances.next().unwrap();
        let pair = distances.get(distance).unwrap();
        if unseen.contains(&pair.0) && unseen.contains(&pair.1) {
            let mut new_circuit: HashSet<usize> = HashSet::new();
            new_circuit.insert(pair.0);
            new_circuit.insert(pair.1);
            seen_groups.push(new_circuit);
            unseen.remove(&pair.0);
            unseen.remove(&pair.1);
            println!("made new connection between {:?} and {:?} with distance {}", pair.0, pair.1, distance);
            connections_made += 1;
        } else if unseen.contains(&pair.0) && !unseen.contains(&pair.1) {
            println!("1 {:?} was already found, merge {:?} with it at distance {}", pair.0, pair.1, distance);
            for group in &mut seen_groups {
                if group.contains(&pair.1) {
                    group.insert(pair.0);
                    unseen.remove(&pair.0);
                    connections_made += 1;
                    break;
                }
            }
        } else if unseen.contains(&pair.1) && !unseen.contains(&pair.0)  {
            println!("0 {:?} was already found, merge {:?} with it at distance {}", pair.1, pair.0, distance);
            for group in &mut seen_groups {
                if group.contains(&pair.0) {
                    group.insert(pair.1);
                    unseen.remove(&pair.1);
                    connections_made += 1;
                    break;
                }
            }
        } else if !unseen.contains(&pair.1) && !unseen.contains(&pair.0){
            let group1 = seen_groups.iter().enumerate()
                .find_map(|(group_idx, group)| {
                    if group.contains(&pair.0) {
                        Some(group_idx)
                    } else {
                        None
                    }
            }).unwrap();
            let group2 = seen_groups.iter().enumerate()
                .find_map(|(group_idx, group)| {
                    if group.contains(&pair.1) {
                        Some(group_idx)
                    } else {
                        None
                    }
            }).unwrap();
            if group1 != group2 {
                let other_group = seen_groups.get(group2).unwrap().clone();
                seen_groups.get_mut(group1).unwrap().extend(other_group);
                
                println!("{:?} and {:?} were already found, merge group {} with {} with it at distance {}", 
                    pair.0, pair.1, group1, group2, distance);
                seen_groups.remove(group2);
                connections_made += 1;
            } else{
                println!("{:?} and {:?} were in the same group: {}-{} with distance {}", 
                    pair.0, pair.1, group1, group2, distance);
                // we already saw this one
                connections_made += 1;
            }
        }else {
            panic!("unaccounted pair {} {}", pair.0, pair.1);
        }
    }
    for group in &seen_groups {
        println!("group len is {}", group.len())
    }

    seen_groups.sort_by(|a,b| b.len().cmp(&a.len()));
    Some(seen_groups.iter().take(3).map(|group| group.len()).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(
            &advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
