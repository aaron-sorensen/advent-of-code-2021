#[derive(Debug)]
pub enum NavDirection {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn part_1() -> u32 {
    get_bearings_old(&get_input())
}

pub fn part_2() -> u32 {
    get_bearings(&get_input())
}

pub fn get_input() -> Vec<NavDirection> {
    let mut nav_instructions: Vec<NavDirection> = Vec::new();

    for line in include_str!("input/day-2.txt").lines() {
        let mut iter = line.split_whitespace();
        let instruction = iter.next().unwrap();
        let value: u32 = iter.next().unwrap().parse::<u32>().unwrap();
        match instruction {
            "forward" => nav_instructions.push(NavDirection::Forward(value)),
            "up" => nav_instructions.push(NavDirection::Up(value)),
            "down" => nav_instructions.push(NavDirection::Down(value)),
            _ => (),
        }
    }
    nav_instructions
}

// Bearings from the first question on day 2
pub fn get_bearings_old(directions: &Vec<NavDirection>) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;

    for direction in directions.iter() {
        match direction {
            NavDirection::Forward(value) => horizontal += value,
            NavDirection::Down(value) => depth += value,
            NavDirection::Up(value) => depth -= value,
        }
    }

    depth * horizontal
}

pub fn get_bearings(directions: &Vec<NavDirection>) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;

    for direction in directions.iter() {
        match direction {
            NavDirection::Forward(value) => {
                horizontal += value;
                depth += aim * value
            }
            NavDirection::Down(value) => aim += value,
            NavDirection::Up(value) => aim -= value,
        }
    }

    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(2102357, part_1());
        assert_eq!(2101031224, part_2());
    }
}
