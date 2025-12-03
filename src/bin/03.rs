advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let banks = parse_input(input);
    let mut total: u32 = 0;

    for bank in banks {
        // find the lowest index of largest value that isn't in the last position
        let mut index1 = 0;
        let mut val1 = 0;
        for idx in 0..bank.len()-1 {
            if bank[idx] > val1 {
                index1 = idx;
                val1 = bank[idx];
            }
        }
        
        // find the largest value who's index is greater than the first index
        let mut index2 = 0;
        let mut val2 = 0;
        for idx in index1+1..bank.len() {
            if bank[idx] > val2 {
                index2 = idx;
                val2 = bank[idx];
            }
            
        }
        total += val1*10 + val2;

    }
    Some(total)
}

fn get_largest_with_room(bank: &Vec<u32>, starting: usize, ending: usize) -> usize {
    let mut index = 0;
    let mut val = 0;
    for idx in starting..=bank.len()-ending {
        if bank[idx] > val {
            index = idx;
            val = bank[idx];
        }
    }
    index
}

pub fn part_two(input: &str) -> Option<usize> {
    let banks = parse_input(input);
    let mut total: usize = 0;

    for bank in banks {

        let mut nth = 12;
        let mut result: Vec<usize> = Vec::new();
        let mut prev_index = 0;
        while nth > 0 {
            let best_option = get_largest_with_room(&bank, prev_index, nth);
            result.push(best_option);
            prev_index = best_option+1;
            nth -= 1;
        }
        let mut num = 0;
        for res in result {
            num += bank[res] as usize;
            num *= 10
        }
        num /= 10;
        total += num;

    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_part_one_example(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_part_two_example(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
