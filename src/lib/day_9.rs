type Grid = Vec<Vec<u8>>;

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

pub fn part_1() -> usize {
    let grid = get_grid();

    let lowest_points = get_lowest_points(&grid);

    lowest_points.iter().fold(0, |mut acc, pos| {
        acc += grid[pos.y as usize][pos.x as usize] as usize + 1;
        acc
    })
}

pub fn part_2() -> usize {
    let grid = get_grid();
    let lowest_points = get_lowest_points(&grid);

    let mut sizes = lowest_points.iter().fold(Vec::new(), |mut acc, pos| {
        let basin_size = get_basin_size(pos, &grid, &mut Vec::new());
        acc.push(basin_size);
        acc
    });

    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn get_basin_size(pos: &Pos, grid: &Grid, basin: &mut Vec<Pos>) -> usize {
    if check_point(pos.x, pos.y, grid) < 9 && !basin.contains(pos) {
        basin.push(pos.clone());
        get_basin_size(
            &Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            grid,
            basin,
        );
        get_basin_size(
            &Pos {
                x: pos.x + 1,
                y: pos.y,
            },
            grid,
            basin,
        );
        get_basin_size(
            &Pos {
                x: pos.x,
                y: pos.y + 1,
            },
            grid,
            basin,
        );
        get_basin_size(
            &Pos {
                x: pos.x - 1,
                y: pos.y,
            },
            grid,
            basin,
        );
    }
    basin.len()
}

fn get_grid() -> Grid {
    include_str!("input/day-9.txt")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(
                line.chars()
                    .into_iter()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
            acc
        })
}

fn get_lowest_points(grid: &Grid) -> Vec<Pos> {
    grid.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            for (x, _item) in row.iter().enumerate() {
                if check_if_lowest(x as isize, y as isize, &grid) {
                    acc.push(Pos {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
            acc
        })
}

fn check_if_lowest(x: isize, y: isize, grid: &Grid) -> bool {
    // Check each of 4 points
    let top = check_point(x, y - 1, grid);
    let right = check_point(x + 1, y, grid);
    let bottom = check_point(x, y + 1, grid);
    let left = check_point(x - 1, y, grid);
    let center = grid[y as usize][x as usize];

    return center < top && center < right && center < bottom && center < left;
}

fn check_point(x: isize, y: isize, grid: &Grid) -> u8 {
    // Out of bounds defaults to higher
    if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
        return 10;
    }
    return grid[y as usize][x as usize];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(532, part_1());
        assert_eq!(1110780, part_2());
    }
}
