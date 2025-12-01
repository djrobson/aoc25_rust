advent_of_code::solution!(1);

struct Position {
    pos: i32,
    clicks: u32,
}

impl Position {
    fn new() -> Self {
        Position{pos: 50, clicks: 0}
    }
    fn adjust(&mut self, amount: i32) {
        if amount == 0 {
            return;
        }

        let starting_at_zero = self.pos == 0;

        if amount > 0 {
            // Moving right: count how many times we pass through 0
            // We pass 0 when we go from 99 to 0, but not if we land exactly on 0
            let target = self.pos + amount;
            let wraps = (target - 1) / 100;
            self.clicks += wraps as u32;
            self.pos = target % 100;
        } else {
            // Moving left: count how many times we pass through 0
            // We pass 0 when we go from 0 to 99
            let abs_amount = (-amount) as i32;
            let mut clicks_to_add = if abs_amount > self.pos {
                1 + (abs_amount - self.pos - 1) / 100
            } else {
                0
            };
            // If we start at 0, don't count leaving it as a click
            if starting_at_zero && clicks_to_add > 0 {
                clicks_to_add -= 1;
            }
            self.clicks += clicks_to_add as u32;
            // Handle negative modulo properly
            self.pos = ((self.pos + amount) % 100 + 100) % 100;
        }

        // If we land on 0, count that as an additional click
        // But if we started at 0 and also ended at 0, don't double count
        if self.pos == 0 && !starting_at_zero {
            self.clicks += 1;
        } else {
            //println!("started and ended at 0!");
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos: Position = Position::new();
    let mut total_zeros: u32 = 0;

    for instruction in input.trim().lines() {
        let direction = instruction.chars().next()?; // First character (R or L)
        let mut amount: i32 = instruction[1..].parse().ok()?; // Rest as number
        //println!("{} {}", direction, amount);
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
    let mut pos: Position = Position::new();

    for instruction in input.trim().lines() {
        let direction = instruction.chars().next()?; // First character (R or L)
        let mut amount: i32 = instruction[1..].parse().ok()?; // Rest as number

        if direction.eq(&'L') {
            amount = -amount;
        }
        pos.adjust(amount);

        // println!("{} to\t{} with {} clicks", amount, pos.pos, pos.clicks);
    }
    Some((pos.clicks)  as u64)
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
        assert_eq!(result, Some(6));
    }
}
