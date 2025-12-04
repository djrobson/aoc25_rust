advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let adjacent_offsets:[(i32,i32);8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    let available_rolls = grid.iter().enumerate()
        .map(|(y, line)| {
            line.iter().enumerate()
                .map(|(x, val)| {
                    if grid[y][x] == b'.' {
                        return 0;
                    }
                    let adjacent_rolls: u64 = adjacent_offsets.iter()
                        .map(|(dy,dx)|{
                            let this_y: i32 = dy+y as i32;
                            let this_x = dx+x as i32;
                            if   this_y >= 0 && this_y < max_y 
                              && this_x >= 0 && this_x < max_x 
                              && grid[this_y as usize][this_x as usize] == b'@' 
                            {
                                1
                            } else {
                                0
                            }
                        }).sum();
                    if adjacent_rolls < 4 {
                        1
                    } else {
                        0
                    }
                }).sum::<u64>()
        }).sum();
    Some(available_rolls)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
