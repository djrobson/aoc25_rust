advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn get_available_rolls( grid: &[Vec<u8>]) -> Vec<(usize,usize)> {
    let mut available_rolls = Vec::new();
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let adjacent_offsets:[(i32,i32);8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    grid.iter().enumerate()
            .for_each(|(y, line)| {
                line.iter().enumerate()
                    .for_each(|(x, val)| {
                        if val == &b'@' {
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
                                available_rolls.push((y,x));
                            };
                        }
                    })
            });
    available_rolls
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let available_rolls = get_available_rolls(&grid);
    Some(available_rolls.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    let mut total_removed: usize = 0;

    let mut available_now = get_available_rolls(&grid);
    while !available_now.is_empty() {
        total_removed += available_now.len();
        available_now.iter().for_each(|(y,x)| grid[*y][*x] = b'.');
        available_now = get_available_rolls(&grid);
    }
    Some(total_removed)
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
