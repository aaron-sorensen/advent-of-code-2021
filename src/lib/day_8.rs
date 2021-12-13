use std::collections::HashSet;

type Signal = HashSet<char>;
type Lookup = [Signal; 10];

pub fn part_1() -> usize {
    count_unique_digits(&get_input())
}

pub fn part_2() -> usize {
    decode_all(&get_input())
}

// If there is exactly one matching element, remove and return it. Else panic.
fn remove_only<T, F>(input: &mut Vec<T>, predicate: F) -> T
where
    T: Clone,
    F: Fn(&&T) -> bool + Copy,
{
    let mut results = input.iter().filter(predicate);
    let result = results.next().expect("no element found").clone();
    assert!(results.next().is_none(), "multiple elements found");

    input.retain(|x| !predicate(&x));

    result
}

fn decode(input: &mut Vec<Signal>) -> Lookup {
    // Easy cases.
    let n1 = remove_only(input, |x| x.len() == 2);
    let n4 = remove_only(input, |x| x.len() == 4);
    let n7 = remove_only(input, |x| x.len() == 3);
    let n8 = remove_only(input, |x| x.len() == 7);

    // 3 is the only 5-segment digit that shares 2 segments with digit 1.
    let n3 = remove_only(input, |x| x.len() == 5 && (*x & &n1).len() == 2);
    let n2 = remove_only(input, |x| x.len() == 5 && (*x & &n4).len() == 2);
    // 5 is the only remaining 5-segment digit.
    let n5 = remove_only(input, |x| x.len() == 5);

    // And so on.
    let n6 = remove_only(input, |x| x.len() == 6 && (*x & &n1).len() == 1);
    let n9 = remove_only(input, |x| x.len() == 6 && (*x & &n4).len() == 4);
    let n0 = remove_only(input, |x| x.len() == 6);

    assert!(input.is_empty());

    [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
}

fn apply(lookup: &Lookup, output: &[Signal]) -> usize {
    output.iter().fold(0, |result, x| {
        result * 10
            + lookup
                .iter()
                .enumerate()
                .find(|(_, y)| x == *y)
                .map(|(index, _)| index)
                .unwrap()
    })
}

fn decode_all(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input
        .iter()
        .map(|line| {
            let output: Vec<Signal> = line.1.iter().map(|x| x.chars().collect()).collect();
            let mut input = line
                .0
                .iter()
                .map(|x| x.chars().collect())
                .collect::<Vec<Signal>>()
                .drain(0..10)
                .collect();

            let lookup = decode(&mut input);
            apply(&lookup, &output)
        })
        .sum()
}

fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
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

fn count_unique_digits(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input.iter().fold(0, |mut acc, line| {
        acc += line
            .1
            .iter()
            .filter(|&l| matches!(l.len(), 2 | 3 | 4 | 7))
            .count();

        acc
    })
}

#[test]
fn check() {
    assert_eq!(473, part_1());
    assert_eq!(1097568, part_2());
}
