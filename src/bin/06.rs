advent_of_code::solution!(6);

enum Operator {
    Add,
    Multiply,
}
struct Homework {
    nums: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

fn parse_input1(input: &str) -> Homework {
    let mut lines = input.lines();
    let mut nums = Vec::new();
    let mut operators = Vec::new();

    lines.for_each(|line| {
        if line.as_bytes()[0] == b'*' || line.as_bytes()[0] == b'+' {

            line.split_whitespace().for_each(
                |oper: &str| {
                    match oper {
                        "+" => operators.push(Operator::Add),
                        "*" => operators.push(Operator::Multiply),
                        _ => {},
                    }
                });
        } else {
            nums.push(line.split_whitespace().flat_map(|num: &str| num.parse()).collect());
        }
    });

    Homework {
        nums: nums,
        operators: operators,
    }
}


fn parse_input2(input: &str) -> Homework {
    let mut lines = input.lines();

    // iterate through the operator row, split all collumns just left of each operator, then do columnwise collection on the numbers
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
    let homework = parse_input1(input);
    let mut total = 0;
    let rows = homework.nums.len();
    let cols = homework.nums[0].len();
    for col in 0..cols {
        total += match homework.operators[col] {
            Operator::Add => {
                let mut col_total = 0;
                for row in 0..rows {
                    col_total += homework.nums[row][col];
                }
                //println!("col {} added to {}", col, col_total);
                col_total
            }
            Operator::Multiply => {
                let mut col_total = 1;
                for row in 0..rows {
                    col_total *= homework.nums[row][col];
                }
                //println!("col {} multiplied to {}", col, col_total);
                col_total
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let homework = parse_input2(input);
    let mut total = 0;
    for col in 0..homework.nums[0].len() {
        total += match homework.operators[col] {
            Operator::Add => {homework.nums[0][col] + homework.nums[1][col] +homework.nums[2][col] +homework.nums[3][col]}
            Operator::Multiply => {homework.nums[0][col] * homework.nums[1][col] *homework.nums[2][col] *homework.nums[3][col]}
        }
    }
    Some(total)
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
        assert_eq!(result, Some(3263827));
    }
}
