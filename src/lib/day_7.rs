pub fn part_1() -> i32 {
    get_position(&get_numbers(), true)
}

pub fn part_2() -> i32 {
    get_position(&get_numbers(), false)
}

fn get_numbers() -> Vec<i32> {
    include_str!("input/day-7.txt")
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

fn get_fuel_cost(crab: i32, position: i32, first_question: bool) -> i32 {
    let distance = i32::abs(crab - position);
    if first_question {
        return distance;
    }
    distance * (distance + 1) / 2
}

fn get_position(crab_positions: &Vec<i32>, first_question: bool) -> i32 {
    let max_pos: &i32 = crab_positions.iter().max().unwrap();
    let mut min_cost = i32::MAX;

    let mut total: i32 = 0;
    for number in 0..*max_pos {
        for crab in crab_positions {
            total += get_fuel_cost(*crab, number, first_question);
        }
        if total < min_cost {
            min_cost = total;
        }
        total = 0;
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(337488, part_1());
        assert_eq!(89647695, part_2());
    }
}
