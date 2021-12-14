pub fn part_1() -> usize {
    let mut grid = get_grid();
    let mut total_flashed: usize = 0;

    // Steps
    for _i in 0..100 {
        total_flashed += step(&mut grid);
    }

    total_flashed
}

pub fn part_2() -> usize {
    let mut grid = get_grid();
    let grid_size = grid.len() * grid[0].len();
    let mut current_step = 0;

    let mut found = false;

    // Steps
    while !found {
        current_step += 1;
        let flashed = step(&mut grid);
        if flashed == grid_size {
            found = true;
        }
    }

    current_step
}

type Grid = Vec<Vec<u8>>;

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

fn build_pos(x: isize, y: isize) -> Pos {
    Pos { x: x, y: y }
}

fn step(grid: &mut Grid) -> usize {
    let mut flashed: Vec<Pos> = Vec::new();

    // Iterate through and tick everything
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            flash(&build_pos(x as isize, y as isize), grid, &mut flashed);
        }
    }

    // Reset flashed to zero
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        }
    }

    flashed.len()
}

fn flash(pos: &Pos, grid: &mut Grid, flashed: &mut Vec<Pos>) -> usize {
    if valid_point(pos, grid) {
        grid[pos.y as usize][pos.x as usize] += 1;
        if grid[pos.y as usize][pos.x as usize] > 9 && !flashed.contains(pos) {
            flashed.push(pos.clone());
            flash(&build_pos(pos.x, pos.y - 1), grid, flashed);
            flash(&build_pos(pos.x + 1, pos.y - 1), grid, flashed);
            flash(&build_pos(pos.x + 1, pos.y), grid, flashed);
            flash(&build_pos(pos.x + 1, pos.y + 1), grid, flashed);
            flash(&build_pos(pos.x, pos.y + 1), grid, flashed);
            flash(&build_pos(pos.x - 1, pos.y + 1), grid, flashed);
            flash(&build_pos(pos.x - 1, pos.y), grid, flashed);
            flash(&build_pos(pos.x - 1, pos.y - 1), grid, flashed);
        }
    }

    flashed.len()
}

fn valid_point(pos: &Pos, grid: &Grid) -> bool {
    pos.x >= 0 && pos.x < grid[0].len() as isize && pos.y >= 0 && pos.y < grid.len() as isize
}

fn get_grid() -> Grid {
    include_str!("input/day-11.txt")
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

#[test]
fn check() {
    assert_eq!(1743, part_1());
    assert_eq!(364, part_2());
}
