advent_of_code::solution!(1);

struct Position {
    pos: i32,
}

impl Position {
    fn new() -> Self {
        Position{pos: 50}
    }
    fn adjust(&mut self, amount: i32) {
        self.pos = self.pos + amount;
        if self.pos < 0 {
            self.pos = self.pos +100;
        }
        self.pos = self.pos % 100;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos: Position = Position::new();
    let mut total_zeros: u32 = 0;

    for instruction in input.trim().lines() {
        let direction = instruction.chars().next()?; // First character (R or L)
        let mut amount: i32 = instruction[1..].parse().ok()?; // Rest as number
        //println!("{} {}", direction, amount);
        amount = amount %100;
        if direction.eq(&'L') {
            amount = -amount;
        }
        pos.adjust(amount);

        //println!("new position is {}", pos.pos);
        if pos.pos == 0 {
            total_zeros += 1;
        }
    }
    Some(total_zeros as u64)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
