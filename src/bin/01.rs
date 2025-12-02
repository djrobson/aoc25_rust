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

        let step = if amount > 0 { 1 } else { -1 };
        let abs_amount = amount.abs();

        // Fast-forward through complete cycles of 100
        if abs_amount >= 100 {
            let full_cycles = abs_amount / 100;
            self.clicks += full_cycles as u32;
        }

        // Simulate the remaining clicks
        let remaining = abs_amount % 100;
        for _ in 0..remaining {
            self.pos = (self.pos + step + 100) % 100;
            if self.pos == 0 {
                self.clicks += 1;
            }
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
