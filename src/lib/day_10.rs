pub fn part_1() -> usize {
    let mut corrupted: Vec<char> = Vec::new();

    for line in include_str!("input/day-10.txt").lines() {
        let mut parsed: Vec<char> = Vec::new();
        for item in line.chars() {
            if is_opening(&item) {
                parsed.push(item);
            } else if get_match(&item) == parsed[parsed.len() - 1] {
                parsed.pop();
            } else {
                corrupted.push(item);
                break;
            }
        }
    }

    corrupted.iter().fold(0, |acc, item| acc + get_score(item))
}

pub fn part_2() -> usize {
    let mut scores: Vec<usize> = Vec::new();

    for line in include_str!("input/day-10.txt").lines() {
        let mut parsed: Vec<char> = Vec::new();
        for item in line.chars() {
            if is_opening(&item) {
                parsed.push(item);
            } else if get_match(&item) == parsed[parsed.len() - 1] {
                parsed.pop();
            } else {
                // Line corrupted, discard
                parsed.clear();
                break;
            }
        }
        if parsed.len() > 0 {
            let mut score: usize = 0;

            while parsed.len() > 0 {
                score *= 5;
                score += get_completion_score(&parsed.pop().unwrap());
            }

            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn is_opening(item: &char) -> bool {
    matches!(item, '(' | '[' | '{' | '<')
}

fn get_match(item: &char) -> char {
    match item {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => 'x',
    }
}

fn get_score(item: &char) -> usize {
    match item {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_completion_score(item: &char) -> usize {
    match item {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

#[test]
fn check() {
    assert_eq!(415953, part_1());
    assert_eq!(2292863731, part_2());
}
