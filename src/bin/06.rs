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
    let lines = input.lines();
    let mut nums = Vec::new();
    let mut operators = Vec::new();

    for line in lines {
        if line.as_bytes()[0] == b'*' || line.as_bytes()[0] == b'+' {

            line.split_whitespace().for_each(
                |oper| {
                    match oper {
                        "+" => operators.push(Operator::Add),
                        "*" => operators.push(Operator::Multiply),
                        _ => {},
                    }
                });
        } else {
            nums.push(line.split_whitespace().flat_map(|num| num.parse()).collect());
        }
    };

    Homework { nums, operators,}
}


fn parse_input2(input: &str) -> Homework {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let operator_row = grid.len()-1; // the operators are in the last row

    let mut col_starts: Vec<usize> = Vec::new();
    let mut operators  = Vec::new();
    let mut nums = Vec::new();

    for col in 0..(grid[operator_row].len()-1) {
        let this_op = grid[operator_row][col];
        if this_op != b' ' {
            col_starts.push(col);
            match this_op {
                b'+' => operators.push(Operator::Add),
                b'*' => operators.push(Operator::Multiply),
                _ => {}
            }
        }
    }
    let grid_last_column = grid[0].len();

    for idx in 0..col_starts.len() {
        let start_col = col_starts[idx];
        let end_col = col_starts.get(idx+1).unwrap_or(&grid_last_column);
        nums.insert(idx, Vec::new());
        for col in start_col..*end_col {
            let num_in_col: String = (0..operator_row)
                .map(|row| grid[row][col] as char)
                .filter(|c| !c.is_whitespace())
                .collect();
            //println!("found the number '{}' in col {}", num_in_col, col);
            if !num_in_col.is_empty() {
                nums[idx].push(num_in_col.parse().unwrap());
            }
        }
    }

    Homework {nums,operators,}
}

pub fn part_one(input: &str) -> Option<u64> {
    let homework = parse_input1(input);
    let rows = homework.nums.len();
    let cols = homework.nums[0].len();

    Some((0..cols)
        .map(|col| {
            match homework.operators[col] {
                Operator::Add => (0..rows).map(|row| homework.nums[row][col]).sum::<u64>(),
                Operator::Multiply => (0..rows).map(|row| homework.nums[row][col]).product::<u64>(),
            }
        })
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let homework = parse_input2(input);
    Some(homework.operators.iter().enumerate()
        .map(|(idx,oper)| {
            match oper {
                Operator::Add => homework.nums[idx].iter().sum::<u64>(),
                Operator::Multiply => homework.nums[idx].iter().product::<u64>(),
            }
        }).sum()
    )
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
