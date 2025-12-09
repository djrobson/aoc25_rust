use std::u64;
use itertools::{self, Itertools};

advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<(i64,i64)> {
    input.lines().map(|line| {
        let mut parts = line.split(",");
        (parts.next().unwrap().parse().unwrap(),parts.next().unwrap().parse().unwrap())
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut biggest_area = 0;
    let corners = parse_input(input);
    for first in 0.. corners.len() {
        for second in first+1..corners.len() {
            let this_area:u64 = (
                ((corners[first].0 -corners[second].0).abs()+1) 
                * ((corners[first].1 -corners[second].1).abs()+1))
                .try_into().unwrap();
            if this_area > biggest_area {
                biggest_area = this_area;
            }
        }
    }
    Some(biggest_area)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut biggest_area = 0;
    let corners = parse_input(input);
    let lines = corners.iter().circular_tuple_windows().take(corners.len()).collect::<Vec<(&(i64,i64),&(i64,i64))>>();
    let mut max_boxes = corners.iter().tuple_combinations().map(|(a,b)| {
        let area = ((a.0 -b.0).abs()+1) * ((a.1 - b.1).abs()+1);
        (a,b,area)
    }).sorted_by_key(|b| b.2).rev();

    let max_box = max_boxes.find(|(a,b,area)| {
        lines.iter().all(|(start,end)| {
            let left_of_rec = a.0.max(b.0) <= start.0.min(end.0);
            let right_of_rec = a.0.min(b.0) >= start.0.max(end.0);
            let above = a.1.max(b.1) <= start.1.min(end.1);
            let below = a.1.min(b.1) >= start.1.max(end.1);
            left_of_rec || right_of_rec || above || below
        })
    }).unwrap();
    Some(max_box.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
