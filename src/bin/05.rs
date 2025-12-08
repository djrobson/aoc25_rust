advent_of_code::solution!(5);
use std::ops::Bound::{self, Excluded, Included};
use unbounded_interval_tree::interval_tree::IntervalTree;

fn parse_input(input: &str) -> (IntervalTree<usize>, Vec<usize>) {
    let mut intervals = IntervalTree::default();
    let mut ingredients: Vec<usize> = Vec::new();
    let mut blank_line_encountered = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            // Check if the line is empty (after trimming whitespace)
            blank_line_encountered = true;
            continue; // Skip the blank line itself
        }

        if blank_line_encountered {
            ingredients.push(line.parse().unwrap());
        } else {
            // split the line before and after the '-'
            if let Some((left_str, right_str)) = line.split_once('-') {
                // Attempt to parse both string slices into u64.
                // The `parse()` method returns a Result, so `ok()` converts it to an Option.
                let low = left_str.parse::<usize>().unwrap();
                let high = right_str.parse::<usize>().unwrap();
                intervals.insert((Included(low), Included(high)));
            } else {
                panic!("unexpected input: {}", line);
            }
        }
    }
    (intervals, ingredients)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (intervals, ingredients) = parse_input(input);
    let fresh_ingredients = ingredients
        .iter()
        .filter(|ingredient| intervals.contains_point(*ingredient))
        .count();
    Some(fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (intervals, _) = parse_input(input);
    let missing = intervals.get_interval_difference::<usize, (Bound<usize>, Bound<usize>)>(&(
        Included(usize::MIN),
        Included(usize::MAX),
    ));
    //println!("we don't cover {:?}", missing);

    let mut included_count = usize::MAX;
    for range in missing {
        match range {
            (Included(low), Excluded(high)) => {
                included_count -= (high - low) - 1;
            }
            (Excluded(low), Excluded(high)) => {
                included_count -= (high - low) - 1;
            }
            (Excluded(low), Included(high)) => {
                included_count -= high - low;
            }
            _ => panic!("Unexpected range: {:?}", range),
        }
    }
    Some(included_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
