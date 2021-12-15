use std::collections::HashMap;

pub fn part_1() -> usize {
    run(false)
}

pub fn part_2() -> usize {
    run(true)
}

fn run(multiple_small_caves: bool) -> usize {
    find_paths(
        vec!["start".to_string()],
        &get_input(),
        &mut 0,
        multiple_small_caves,
    )
}

fn get_input() -> HashMap<String, Vec<String>> {
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    for line in include_str!("input/day-12.txt").lines() {
        let split = line
            .split("-")
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        nodes.entry(split[0].clone()).or_insert(Vec::new());
        nodes.entry(split[1].clone()).or_insert(Vec::new());

        if !nodes[&split[0]].contains(&split[1]) {
            nodes.get_mut(&split[0]).unwrap().push(split[1].clone());
        }
        if !nodes[&split[1]].contains(&split[0]) {
            nodes.get_mut(&split[1]).unwrap().push(split[0].clone());
        }
    }
    nodes
}

fn find_paths(
    path: Vec<String>,
    nodes: &HashMap<String, Vec<String>>,
    paths: &mut usize,
    multiple_small_caves: bool,
) -> usize {
    if path[path.len() - 1] == "end" {
        *paths += 1;
    } else {
        for connection in &nodes[&path[path.len() - 1]] {
            let is_uppercase = connection.chars().all(|c| c.is_ascii_uppercase());
            let already_visited = path.contains(connection);
            if connection != "start" && (is_uppercase || (!already_visited || multiple_small_caves))
            {
                let mut new_path = path.clone();
                new_path.push(connection.to_string());
                if is_uppercase || !already_visited {
                    find_paths(new_path, nodes, paths, multiple_small_caves);
                } else if !is_uppercase && already_visited && multiple_small_caves {
                    find_paths(new_path, nodes, paths, false);
                }
            }
        }
    }
    *paths
}

#[test]
fn check() {
    assert_eq!(4186, part_1());
    assert_eq!(92111, part_2());
}
