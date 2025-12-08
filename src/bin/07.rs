advent_of_code::solution!(7);
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spot {
    Empty,
    //Start,
    Split,
    Beam(usize),
}
impl fmt::Debug for Spot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Spot::Empty => write!(f, "."),
            //Start,
            Spot::Split => write!(f, "^"),
            Spot::Beam(count) => write!(f, "{}", count % 10),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .map(|line: &str| {
            line.chars()
                .map(|c| match c {
                    '.' => Spot::Empty,
                    'S' => Spot::Beam(1),
                    '^' => Spot::Split,
                    _ => panic!("unexpected input {}", c),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_splits = 0;
    let grid = parse_input(input);
    let line_len = grid[0].len();
    let mut line_idx = 1;
    let mut this_line: Vec<Spot> = grid[0].clone();
    let mut next_line: Vec<Spot>;
    while line_idx < grid.len() {
        next_line = vec![Spot::Empty; line_len];
        for spot in 0..line_len {
            if let Spot::Beam(_count) = this_line[spot] {
                // check if any beams run into any splitters
                if grid[line_idx][spot] == Spot::Split {
                    total_splits += 1;
                    next_line[spot - 1] = Spot::Beam(1);
                    next_line[spot + 1] = Spot::Beam(1);
                } else {
                    next_line[spot] = Spot::Beam(1);
                }
            }
        }
        line_idx += 1;
        std::mem::swap(&mut this_line, &mut next_line);
    }
    Some(total_splits)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let line_len = grid[0].len();
    let mut line_idx = 1;
    let mut this_line: Vec<Spot> = grid[0].clone();
    let mut next_line: Vec<Spot>;
    while line_idx < grid.len() {
        next_line = vec![Spot::Empty; line_len];
        //println!("this line {:?}", this_line);
        for spot in 0..line_len {
            if let Spot::Beam(count) = this_line[spot] {
                // check if any beams run into any splitters
                if grid[line_idx][spot] == Spot::Split {
                    if let Spot::Beam(bottom_left_count) = next_line[spot - 1] {
                        next_line[spot - 1] = Spot::Beam(bottom_left_count + count);
                    } else {
                        next_line[spot - 1] = Spot::Beam(count);
                    }
                    next_line[spot + 1] = Spot::Beam(count);
                } else if let Spot::Beam(straight_down_count) = next_line[spot] {
                    // check if a split just to our left needs to merge with one going straight down
                    next_line[spot] = Spot::Beam(count + straight_down_count);
                } else {
                    next_line[spot] = Spot::Beam(count);
                }
            }
        }
        line_idx += 1;
        std::mem::swap(&mut this_line, &mut next_line);
    }
    Some(
        this_line
            .iter()
            .map(|spot: &Spot| match spot {
                Spot::Empty => 0,
                Spot::Split => panic!("there was a split in the last line"),
                Spot::Beam(count) => *count,
            })
            .sum(),
    )
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
        assert_eq!(result, Some(40));
    }
}
