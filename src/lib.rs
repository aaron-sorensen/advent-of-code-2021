use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub test: Vec<usize>,
    pub real: Vec<usize>,
}

pub fn get_increment_window_count(sweep: &Vec<usize>, window: usize) -> usize {
    let windows: Vec<usize> =
        sweep
            .iter()
            .skip(window - 1)
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, _depth)| {
                let mut result: usize = 0;
                for n in 0..window {
                    result += sweep[i + n]
                }
                acc.push(result);
                acc
            });
    get_increment_count(&windows)
}

pub fn get_increment_count(sweep: &Vec<usize>) -> usize {
    sweep.iter().skip(1).enumerate().fold(0, |acc, (i, depth)| {
        if depth > &sweep[i] {
            return acc + 1;
        }
        acc
    })
}

pub fn get_input(file: &str) -> Input {
    let path = format!("./input/{}.json", file);
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: Input = serde_json::from_str(&data).expect("Unable to parse");
    res
}
