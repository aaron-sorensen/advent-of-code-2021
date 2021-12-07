use std::fs;
use std::io::{BufRead, BufReader};

pub fn get_numbers(file: &str) -> Vec<u64> {
    let path = format!("./input/{}.txt", file);

    let f = fs::File::open(path).expect("Unable to open file");
    let mut f = BufReader::new(f);

    let mut buf = String::new();

    f.read_line(&mut buf).expect("reading line failed");

    buf.split(",")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

pub fn calculate_growth(starting_fish: &Vec<u64>, days: usize) -> u64 {
    let mut buckets: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut day = 1;

    for fish in starting_fish {
        buckets[*fish as usize] += 1;
    }

    while day <= days {
        let new_fish = buckets.remove(0);
        buckets.push(new_fish);
        buckets[6] += new_fish;
        day += 1;
    }

    buckets.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "day-6";

    #[test]
    fn it_maintains_correct_answers() {
        let numbers = get_numbers(DAY);
        assert_eq!(350605, calculate_growth(&numbers, 80));
        assert_eq!(1592778185024, calculate_growth(&numbers, 256));
    }
}
