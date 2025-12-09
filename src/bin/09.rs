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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
