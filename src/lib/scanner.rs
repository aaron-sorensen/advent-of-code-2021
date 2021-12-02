use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ScannerSweep {
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

pub fn get_input(file: &str) -> ScannerSweep {
    let path = format!("./input/{}.json", file);
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: ScannerSweep = serde_json::from_str(&data).expect("Unable to parse");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "day-1";

    #[test]
    fn it_counts_increments() {
        let input = get_input(DAY);
        let result = get_increment_count(&input.test);
        assert_eq!(7, result);
    }

    #[test]
    fn it_counts_window_increments() {
        let input = get_input(DAY);
        let result = get_increment_window_count(&input.test, 3);
        assert_eq!(5, result);
    }

    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input(DAY);
        let increments = get_increment_count(&input.real);
        let window_increments = get_increment_window_count(&input.real, 3);
        assert_eq!(1466, increments);
        assert_eq!(1491, window_increments);
    }
}
