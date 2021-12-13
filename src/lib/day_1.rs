type Sweeps = Vec<usize>;

pub fn part_1() -> usize {
    get_increment_count(&get_input())
}

pub fn part_2() -> usize {
    get_increment_window_count(&get_input(), 3)
}

fn get_increment_window_count(sweeps: &Sweeps, window: usize) -> usize {
    let windows: Vec<usize> =
        sweeps
            .iter()
            .skip(window - 1)
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, _depth)| {
                let mut result: usize = 0;
                for n in 0..window {
                    result += sweeps[i + n]
                }
                acc.push(result);
                acc
            });
    get_increment_count(&windows)
}

fn get_increment_count(sweeps: &Sweeps) -> usize {
    sweeps
        .iter()
        .skip(1)
        .enumerate()
        .fold(0, |acc, (i, depth)| {
            if depth > &sweeps[i] {
                return acc + 1;
            }
            acc
        })
}

fn get_input() -> Sweeps {
    include_str!("input/day-1.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(1466, part_1());
        assert_eq!(1491, part_2());
    }
}
