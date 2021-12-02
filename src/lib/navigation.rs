use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum NavDirection {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn get_input(file: &str) -> Vec<NavDirection> {
    let mut nav_instructions: Vec<NavDirection> = Vec::new();
    let path = format!("./input/{}.txt", file);

    let f = fs::File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
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

pub fn get_bearings(directions: Vec<NavDirection>) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "day-2";

    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input(DAY);
        let bearings = get_bearings(input);
        assert_eq!(2102357, bearings);
    }
}
