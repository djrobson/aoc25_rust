advent_of_code::solution!(6);

enum Operator {
    Add,
    Multiply,
}
struct Homework {
    nums: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

fn parse_input(input: &str) -> Homework {
    let mut lines = input.lines();
    let nums1: Vec<u64> = lines.next().unwrap().split_whitespace().flat_map(|num: &str| num.parse()).collect();
    let nums2: Vec<u64> = lines.next().unwrap().split_whitespace().flat_map(|num: &str| num.parse()).collect();
    let nums3: Vec<u64> = lines.next().unwrap().split_whitespace().flat_map(|num: &str| num.parse()).collect();
    let nums4: Vec<u64> = lines.next().unwrap().split_whitespace().flat_map(|num: &str| num.parse()).collect();
    let operators: Vec<Operator> = lines.next().unwrap().split_whitespace().flat_map(
        |oper: &str| {
            match oper {
                "+" => Some(Operator::Add),
                "*" => Some(Operator::Multiply),
                _ => None,
            }
        }).collect();

    Homework {
        nums: vec!(nums1, nums2, nums3, nums4),
        operators: operators,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let homework = parse_input(input);
    let mut total = 0;
    for col in 0..homework.nums[0].len() {
        total += match homework.operators[col] {
            Operator::Add => {homework.nums[0][col] + homework.nums[1][col] +homework.nums[2][col] +homework.nums[3][col]}
            Operator::Multiply => {homework.nums[0][col] * homework.nums[1][col] *homework.nums[2][col] *homework.nums[3][col]}
        }
    }
    Some(total)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
