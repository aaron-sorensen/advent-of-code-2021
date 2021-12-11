#[derive(Debug, Clone)]
pub struct VentLine {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub fn get_input() -> Vec<VentLine> {
    let mut lines: Vec<VentLine> = Vec::new();

    for line in include_str!("input/day-5.txt").lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();

        let start = split[0]
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let end = split[2]
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        lines.push(VentLine {
            start: (start[0], start[1]),
            end: (end[0], end[1]),
        })
    }
    lines
}

pub fn generate_map(lines: &Vec<VentLine>, diagonals: bool) -> Vec<Vec<usize>> {
    let mut map = Vec::new();

    for y in 0..1000 {
        map.push(Vec::new());
        for _x in 0..1000 {
            map[y].push(0)
        }
    }

    for line in lines {
        if line.start.0 == line.end.0 || line.start.1 == line.end.1 {
            // Vertical/Horizontal Line
            let min_x = line.start.0.min(line.end.0);
            let max_x = line.start.0.max(line.end.0) + 1;
            let min_y = line.start.1.min(line.end.1);
            let max_y = line.start.1.max(line.end.1) + 1;

            for x in min_x..max_x {
                for y in min_y..max_y {
                    map[y][x] += 1;
                }
            }
        } else if diagonals {
            // Diagonal Line
            let min_x = line.start.0.min(line.end.0);
            let max_x = line.start.0.max(line.end.0) + 1;
            let start_y = if min_x == line.start.0 {
                line.start.1
            } else {
                line.end.1
            };
            let end_y = if max_x == line.start.0 + 1 {
                line.start.1
            } else {
                line.end.1
            };

            let slope: i32 = if start_y < end_y { 1 } else { -1 };
            let mut y = start_y as i32;

            for x in min_x..max_x {
                map[y as usize][x] += 1;
                y += slope;
            }
        }
    }
    map
}

pub fn elevation_count(map: &Vec<Vec<usize>>, elevation: usize) -> usize {
    let mut result: usize = 0;
    for row in map {
        for item in row {
            if item >= &elevation {
                result += 1
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maintains_correct_answers() {
        let input = get_input();
        let map_1 = generate_map(&input, false);
        let map_2 = generate_map(&input, true);
        assert_eq!(6283, elevation_count(&map_1, 2));
        assert_eq!(18864, elevation_count(&map_2, 2));
    }
}
