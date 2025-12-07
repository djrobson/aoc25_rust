advent_of_code::solution!(7);

#[derive(Clone,Copy,PartialEq,Eq, Debug)]
enum Spot {
    Empty,
    //Start,
    Split,
    Beam,
}

fn parse_input(input: &str) -> Vec<Vec<Spot>> {
    input.lines().map(|line: &str| {
        line.chars().map(|c| {
            let spot_val = 
            match c {
                '.' => Spot::Empty,
                'S' => Spot::Beam,
                '^' => Spot::Split,
                _ => panic!("unexpected input {}", c)
            };
            //println!("{}->{:?}", c, spot_val);
            spot_val
        }).collect()
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_splits = 0;
    let grid = parse_input(input);
    let line_len = grid[0].len();
    let mut line_idx = 1;
    let mut this_line: Vec<Spot> = grid[0].clone();
    let mut next_line: Vec<Spot> = vec![Spot::Empty; line_len];
    while line_idx < grid.len() {
        next_line = vec![Spot::Empty; line_len];
        for spot in 0..line_len {
            if this_line[spot] == Spot::Beam {
                // check if any beams run into any splitters
                if grid[line_idx][spot] == Spot::Split {
                    total_splits += 1;
                    next_line[spot-1] = Spot::Beam;
                    next_line[spot+1] = Spot::Beam;
                }   else {
                    next_line[spot] = Spot::Beam;
                }
            } 
        }
        line_idx += 1;
        std::mem::swap(&mut this_line, &mut next_line);
    }
    Some(total_splits)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
