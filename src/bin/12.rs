advent_of_code::solution!(12);

struct Tile {
    shape: [[u8;3];3],
    count: u8
}

struct Grid {
    x: usize,
    y: usize,
    counts: [usize;6]
}

fn parse_input(input: &str) -> (Vec<Tile>, Vec<Grid>) {
    let mut tiles = Vec::new();
    let mut grids = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check if it's a grid definition (contains 'x' and ':')
        if line.contains('x') && line.contains(':') {
            // Parse grid: "4x4: 0 0 0 0 2 0"
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = parts[0].split('x').collect();
            let x = dims[0].trim().parse().unwrap();
            let y = dims[1].trim().parse().unwrap();

            let counts_str = parts[1].trim();
            let count_nums: Vec<usize> = counts_str
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            // Convert to fixed-size array
            let mut counts = [0; 6];
            for (idx, &val) in count_nums.iter().enumerate() {
                if idx < 6 {
                    counts[idx] = val;
                }
            }

            grids.push(Grid { x, y, counts });
            i += 1;
        }
        // Check if it's a tile definition (ends with ':')
        else if line.ends_with(':') {
            // Parse tile
            let mut shape = [[0u8; 3]; 3];
            let mut count = 0;

            // Read next 3 lines for the shape
            for row in 0..3 {
                i += 1;
                if i < lines.len() {
                    let shape_line = lines[i];
                    for (col, ch) in shape_line.chars().enumerate() {
                        if col < 3 {
                            shape[row][col] = if ch == '#' { count+=1; 1 } else { 0 };
                        }
                    }
                }
            }

            tiles.push(Tile { shape, count });
            i += 1;
        } else {
            // Skip empty lines
            i += 1;
        }
    }

    (tiles, grids)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut tiles, mut grids) = parse_input(input);


    /*let possible_grids_by_count: usize = grids.iter().filter(|grid| {
            let needed_spots = std::iter::zip(&tiles, grid.counts).map(|(tile, tile_count)| {
                tile.count as usize * tile_count
            }).sum::<usize>();
            let grid_max_spots = grid.x*grid.y;           
            needed_spots < grid_max_spots
        })
        .count();*/

    // this is a high end of possible grids without any attempt to overlap them
    let possible_unpacked_grids: usize = grids.iter().filter(|grid| {
        let unpacked_spots = (grid.x/3) * (grid.y/3);
        let total_tiles = grid.counts.iter().sum();
        unpacked_spots >= total_tiles
    }).count();
    Some(possible_unpacked_grids)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
