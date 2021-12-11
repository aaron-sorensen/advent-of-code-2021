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

pub fn get_input() -> Vec<usize> {
    include_str!("input/day-1.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input();
        let increments = get_increment_count(&input);
        let window_increments = get_increment_window_count(&input, 3);
        assert_eq!(1466, increments);
        assert_eq!(1491, window_increments);
    }
}
