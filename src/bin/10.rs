advent_of_code::solution!(10);
use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<u16>, Vec<u16>, Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let mut lights: Vec<u16> = Vec::new();
    let mut buttons: Vec<Vec<u16>> = Vec::new();
    let mut joltage: Vec<Vec<u16>> = Vec::new();
    let mut masks: Vec<u16> = Vec::new();

    for line in input.lines() {
        let mut splits: Vec<&str> = line.split(" ").collect();
        let my_light_as_string: String = splits[0].as_bytes().iter().flat_map(|b| {
            match b {
                b'.' => Some('0'),
                b'#' => Some('1'),
                _ => None
            }
        }).rev().collect();
        if my_light_as_string.len() > 15 {
            panic!("my_light_as_string is too long: {}", my_light_as_string.len());
        }
        let mask: u16 = u16::pow(2, my_light_as_string.len() as u32 + 1) -1;
        masks.push(mask);
        lights.push(u16::from_str_radix(&my_light_as_string,2).unwrap());

        let mut my_buttons: Vec<u16> = Vec::new();
        for button in 1..splits.len()-1 {
            let mut this_button = 0;
            let this_button_string = splits[button];
            let this_button_string_stripped = &this_button_string[1..this_button_string.len()-1];
            this_button_string_stripped.split(",").for_each(|b| {
                let this_buttom_light_toggle = b.parse::<u8>().unwrap();
                this_button |= 1<<this_buttom_light_toggle;
            });
            my_buttons.push(this_button);
        }
        buttons.push(my_buttons);

        let this_joltages = splits[splits.len()-1];
        let this_joltages_stripped = &this_joltages[1..this_joltages.len()-1];
        let this_joltage_nums: Vec<u16> = this_joltages_stripped.split(",").map(|b| {
                //println!("{}", b);
                b.parse::<u16>().unwrap()
            }).collect();

        joltage.push(this_joltage_nums);
    };

    (lights, masks, buttons, joltage)
}

fn find_min_pushes(goal_lights: u16, mask: u16, mach_buttons: &Vec<u16>, cur_lights: u16, cur_depth: u16, seen_before: &mut HashSet<u16>) -> Option<u16> {
    if seen_before.contains(&cur_lights) {
        return None;
    }
    if cur_lights == goal_lights {
        return Some(cur_depth);
    }
    seen_before.insert(cur_lights);

    let shortest = mach_buttons.iter().flat_map( |button| find_min_pushes(goal_lights, mask, mach_buttons, (cur_lights ^ button) & mask, cur_depth + 1, seen_before) ).min();
    
    shortest
}

pub fn part_one(input: &str) -> Option<u64> {
    let (lights, masks, buttons, _joltages) = parse_input(input);
    //println!("{:?} {} {:?} {:?}", lights, mask, buttons, joltages);
    let total_machines = lights.len();
    let mut total_button_pushes = 0;
    for machine in 0..total_machines {
        let mach_lights = &lights[machine];
        let mach_mask = masks[machine];
        let mach_buttons = &buttons[machine];

        let mut seen_before: HashSet<u16> = HashSet::new();
        let min_pushes = find_min_pushes(*mach_lights, mach_mask, &mach_buttons, 0, 0, &mut seen_before);
        println!("{}: {}&{} - {:?} = {:?}", machine, mach_lights, mach_mask, mach_buttons, min_pushes);
        total_button_pushes += min_pushes.unwrap() as u64;
    }
    Some(total_button_pushes)
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
