use std::collections::HashSet;

struct Input {
    grid: HashSet<(usize, usize)>,
    folds: Vec<(String, usize)>,
}

pub fn part_1() -> usize {
    let input = get_input();
    fold(Input {
        grid: input.grid,
        folds: vec![input.folds[0].clone()],
    })
    .len()
}

pub fn part_2() {
    render_grid(fold(get_input()));
}

fn fold(mut input: Input) -> HashSet<(usize, usize)> {
    for fold in input.folds {
        let mut result: HashSet<(usize, usize)> = HashSet::new();

        if fold.0.eq("x") {
            for dot in input.grid.iter() {
                if dot.0 > fold.1 {
                    let shift = dot.0 - fold.1;
                    result.insert((fold.1 - shift, dot.1));
                } else {
                    result.insert(dot.clone());
                }
            }
        } else {
            for dot in input.grid.iter() {
                if dot.1 > fold.1 {
                    let shift = dot.1 - fold.1;
                    result.insert((dot.0, fold.1 - shift));
                } else {
                    result.insert(dot.clone());
                }
            }
        }
        input.grid = result;
    }
    input.grid
}

fn render_grid(grid: HashSet<(usize, usize)>) {
    let width: usize = *grid
        .iter()
        .map(|x| x.0)
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
        + 1;
    let height: usize = *grid
        .iter()
        .map(|x| x.1)
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
        + 1;

    let mut result = vec![vec![" "; width]; height];

    for dot in grid {
        result[dot.1][dot.0] = "#";
    }

    for line in result {
        println!("{}", line.join(""));
    }
}

fn get_input() -> Input {
    let mut grid: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(String, usize)> = Vec::new();

    for line in include_str!("input/day-13.txt").lines() {
        if line.contains("fold") {
            let split = line
                .split_whitespace()
                .last()
                .unwrap()
                .split("=")
                .collect::<Vec<&str>>();

            folds.push((split[0].to_string(), split[1].parse::<usize>().unwrap()));
        } else if line.len() > 0 {
            let split = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            grid.insert((split[0], split[1]));
        }
    }
    Input {
        grid: grid,
        folds: folds,
    }
}

#[test]
fn check() {
    assert_eq!(675, part_1());
}
