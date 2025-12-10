advent_of_code::solution!(10);
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

fn parse_input(input: &str) -> (Vec<u16>, Vec<u16>, Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let mut lights: Vec<u16> = Vec::new();
    let mut buttons: Vec<Vec<u16>> = Vec::new();
    let mut joltage: Vec<Vec<u16>> = Vec::new();
    let mut masks: Vec<u16> = Vec::new();

    for line in input.lines() {
        let splits: Vec<&str> = line.split(" ").collect();
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

fn find_min_pushes(goal_lights: u16, mask: u16, mach_buttons: &Vec<u16>) -> u32  {

    (0..u16::pow(2,mach_buttons.len() as u32)).flat_map(|button_combo|{
        let mut result = 0;
        for button in 0..mach_buttons.len() {
            if button_combo & (1<<button) != 0 {
                result = (result ^ mach_buttons[button]) & mask;
            }
        }
        if goal_lights == result {
            Some(button_combo.count_ones())
         } else {
            None
         }
    }).min().unwrap()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_min_joltage_pushes_memo(goal_joltage: &Vec<u16>, mach_buttons: &Vec<u16>, cur_joltage: Vec<u16>, cur_depth: u16, seen_before: &mut HashMap<u64, u16>, cur_shortest_seen: u16) -> Option<u16> {

    // jump out if we've been here in fewer steps already
    if seen_before.contains_key(&calculate_hash(&cur_joltage)) {
        let prev_best = seen_before.get(&calculate_hash(&cur_joltage)).unwrap();
        if *prev_best <= cur_depth {
            //println!("got to the same place at a depth worse or equal to prev best");
            return None;
        }
    }

    // we found a match
    if cur_joltage.eq(goal_joltage) {
        println!("found something at depth {}", cur_depth);
        return Some(cur_depth);
    }

    if cur_depth == cur_shortest_seen {
        // we're about to be too long, stop now
        println!("my depth about to be worse than prev best");
        return None;
    }

    // remember that we've been here before
    seen_before.insert(calculate_hash(&cur_joltage), cur_depth);

    if seen_before.len() % 10000 == 0 {
        println!("we've seen {} states", seen_before.len());
    }

    // at any step we can hit 1 button
    let mut my_shortest_seen = cur_shortest_seen;
    let shortest = mach_buttons.iter().flat_map( |button| {

        let mut new_joltage = cur_joltage.clone();
        // update all joltages based on the chosen button
        for battery in 0..new_joltage.len() {
            if button & (1<<battery) != 0 {
                new_joltage[battery] += 1;
                // if we grew too much, then quit
                if new_joltage[battery] > goal_joltage[battery] {
                    //println!("we grew too big");
                    return None;
                }
            }
        }
        let result = find_min_joltage_pushes_memo(goal_joltage, mach_buttons, new_joltage, cur_depth + 1, seen_before, my_shortest_seen);
        if let Some(found) = result {
            if found < my_shortest_seen {
                // skip anything worse than our current best
                println!("found new best: {}", found);
                my_shortest_seen = found;
            }
        }

        result
    }).min();
    
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

        //let mut seen_before: HashSet<u16> = HashSet::new();
        //let min_pushes = find_min_pushes(*mach_lights, mach_mask, &mach_buttons, 0, 0, &mut seen_before);
        let min_pushes = find_min_pushes(*mach_lights, mach_mask, &mach_buttons);
        //println!("{}: {}&{} - {:?} = {:?}", machine, mach_lights, mach_mask, mach_buttons, min_pushes);
        total_button_pushes += min_pushes as u64;
    }
    Some(total_button_pushes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_lights, _masks, buttons, joltages) = parse_input(input);
    //println!("{:?} {:?}", buttons, joltages);
    let total_machines = joltages.len();
    let mut total_button_pushes = 0;
    for machine in 0..total_machines {
        let mach_joltages = &joltages[machine];
        let mach_buttons = &buttons[machine];

        let mut seen_before: HashMap<u64,u16> = HashMap::new();
        let max_pushes = mach_joltages.iter().sum::<u16>() +1;
        let min_pushes = find_min_joltage_pushes_memo(mach_joltages, &mach_buttons, vec![0;mach_joltages.len()], 0, &mut seen_before, max_pushes);
        println!("{}: {:?} - {:?} = {:?}", machine, mach_joltages, mach_buttons, min_pushes);
        total_button_pushes += min_pushes.unwrap() as u64;
    }
    Some(total_button_pushes)
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two("[.##......] (0,1,3,4,6,7,8) (1,2,3,5,6,8) (0,1) (3,5,6,7) (2,5,7) (1,2,3,4,5,7,8) (7) (0,1,3) (0,3,7) (1,4,6) {36,63,29,56,28,48,43,52,23}");
        assert_eq!(result, Some(60));
    }
}
