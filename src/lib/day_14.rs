use std::collections::HashMap;

struct Input {
    template: Vec<char>,
    insertions: HashMap<String, char>,
}

pub fn part_1() -> usize {
    let input = get_input();
    let mut map = build_map(&input);

    for _i in 0..10 {
        map = step(map, &input.insertions);
    }

    get_score(map)
}

pub fn part_2() -> usize {
    let input = get_input();
    let mut map = build_map(&input);

    for _i in 0..40 {
        map = step(map, &input.insertions);
    }

    get_score(map)
}

fn get_score(map: HashMap<String, usize>) -> usize {
    let mut tallies: HashMap<char, usize> = HashMap::new();
    *tallies.entry('K').or_insert(0) += 1;

    for pair in map {
        let letter = pair.0.chars().collect::<Vec<char>>()[0];
        *tallies.entry(letter).or_insert(0) += pair.1;
    }

    let max = *tallies.iter().map(|x| x.1).max().unwrap();
    let min = *tallies.iter().map(|x| x.1).min().unwrap();

    max - min
}

fn build_map(input: &Input) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for i in 0..input.template.len() - 1 {
        *map.entry(input.template[i].to_string() + &input.template[i + 1].to_string())
            .or_insert(0) += 1;
    }

    map
}

fn step(map: HashMap<String, usize>, insertions: &HashMap<String, char>) -> HashMap<String, usize> {
    let mut new_map: HashMap<String, usize> = HashMap::new();

    for pair in map {
        let insertion = insertions.get(&pair.0).unwrap();
        let pair_1 = pair.0.chars().collect::<Vec<char>>()[0].to_string() + &insertion.to_string();
        let pair_2 = insertion.to_string() + &pair.0.chars().collect::<Vec<char>>()[1].to_string();

        *new_map.entry(pair_1).or_insert(0) += pair.1;
        *new_map.entry(pair_2).or_insert(0) += pair.1;
    }

    new_map
}

fn get_input() -> Input {
    let template = include_str!("input/day-14.txt")
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let mut insertions: HashMap<String, char> = HashMap::new();

    for line in include_str!("input/day-14.txt").lines().skip(2) {
        let split = line.split(" -> ").collect::<Vec<_>>();
        insertions.insert(split[0].to_string(), split[1].chars().last().unwrap());
    }

    Input {
        template: template,
        insertions: insertions,
    }
}

#[test]
fn check() {
    assert_eq!(2345, part_1());
    assert_eq!(2432786807053, part_2());
}
