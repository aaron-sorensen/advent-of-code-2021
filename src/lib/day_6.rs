pub fn part_1() -> u64 {
    calculate_growth(&get_numbers(), 80)
}

pub fn part_2() -> u64 {
    calculate_growth(&get_numbers(), 256)
}

fn get_numbers() -> Vec<u64> {
    include_str!("input/day-6.txt")
        .split(",")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect()
}

fn calculate_growth(starting_fish: &Vec<u64>, days: usize) -> u64 {
    let mut buckets: Vec<u64> = vec![0; 9];
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

    #[test]
    fn check() {
        assert_eq!(350605, part_1());
        assert_eq!(1592778185024, part_2());
    }
}
