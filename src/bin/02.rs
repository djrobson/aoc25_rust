advent_of_code::solution!(2);

struct Range {
    bottom: u64,
    top: u64,
}

fn parse_input(input: &str) -> Vec<Range> {
    use regex::Regex;

    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    re.captures_iter(input)
        .map(|cap| Range {
            bottom: cap[1].parse().unwrap(),
            top: cap[2].parse().unwrap(),
        })
        .collect()
}

fn find_part_one_in_range(bottom: u64, top: u64) -> u64 {
    let mut this_sum: u64 = 0;
    let bot_decimal = count_decimal(&bottom);
    let top_decimal = count_decimal( &top);
    let bot_odd: bool = bot_decimal & 1 == 1;
    let top_odd: bool = top_decimal & 1 == 1;
    let mut my_bot = bottom;
    let mut my_top= top;
    //println!("{}-{} has {},{} decimal places and a range of {}", range.bottom, range.top, 
    //    bot_decimal, top_decimap, range_size);
    // if top and bot decimal are odd then we're done
    if top_odd && bot_odd {
        return 0;
    } else  if bot_odd {
        // if bot decimal is odd and top is even, then increase bot to the lowest even decimal and continue
        my_bot = 10_u64.pow(top_decimal-1);
    } else if top_odd {
        // if top decimal is odd, then decrease top decimal to just below that decimal level and continue
        my_top = 10_u64.pow(top_decimal-1) -1;
    }
    // for each bot decimal, mask out the lowest half of the number, double it and see if that's smaller top
    let decimal_shift = 10_u64.pow(top_decimal/2);
    let mut this_bot : u64 = my_bot / decimal_shift;
    let mut possible_num = this_bot * decimal_shift + this_bot;
    if possible_num < my_bot {
        // truncating could have made us below the minimum, try moving up
        this_bot += 1;
        possible_num = this_bot * decimal_shift + this_bot;
    }
    while possible_num >= my_bot && possible_num <= my_top {
        this_sum += possible_num;
        this_bot += 1;
        possible_num = this_bot * decimal_shift + this_bot;
    }
    this_sum
}

pub fn count_decimal(num: &u64)->u32 {
    let mut result = 1;
    let mut num_copy: u64 = *num;
    while num_copy >= 10 {
        num_copy /= 10;
        result += 1;
    }
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(&input);
    let total: u64 = 
        ranges.iter().map(|range|  {
            find_part_one_in_range(range.bottom, range.top)
        }).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_one_decimal_odd() {
        let result = count_decimal(&998);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_one_decimal_even() {
        let result = count_decimal(&1012);
        assert_eq!(result, 4);
    }

    #[rstest]
    #[case("11-22", 33)]
    #[case("95-115", 99)]
    #[case("998-1012", 1010)]
    #[case("1188511880-1188511890", 1188511885)]
    #[case("222220-222224", 222222)]
    #[case("1698522-1698528", 0)]
    #[case("446443-446449", 446446)]
    #[case("38593856-38593862", 38593859)]
    #[case("565653-565659",  0)]
    #[case("824824821-824824827", 0)]
    #[case("2121212118-2121212124", 0)]
    fn test_part_one_example(#[case] input: &str, #[case] expected: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("132454-182049", 7700693)]
    #[case("42382932-42449104", 296899687)]
    #[case("685933-804865", 88743655)]
    #[case("5330496-5488118", 0)]
    #[case("21-41", 55)]
    #[case("289741-376488", 28999971)]
    #[case("220191-245907", 6051045)]
    #[case("49-70", 121)]
    #[case("6438484-6636872", 0)]
    #[case("2-20", 11)]
    #[case("6666660113-6666682086", 6666666666)]
    fn test_part_one_inputs(#[case] input: &str, #[case] expected: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
