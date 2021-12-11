pub fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    include_str!("input/day-8.txt")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            let split: Vec<String> = line.split("|").map(|s| s.to_string()).collect();
            let left: Vec<String> = split[0].split_whitespace().map(|s| s.to_string()).collect();
            let right: Vec<String> = split[1].split_whitespace().map(|s| s.to_string()).collect();
            acc.push((left, right));
            acc
        })
}

pub fn count_unique_digits(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input.iter().fold(0, |mut acc, line| {
        acc += line
            .1
            .iter()
            .map(|wire| wire.len())
            .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
            .count();

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input();
        assert_eq!(473, count_unique_digits(&input));
    }
}
