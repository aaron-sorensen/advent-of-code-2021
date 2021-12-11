mod bingo;
mod crabs;
mod diagnostic;
mod lanternfish;
mod navigation;
mod scanner;
mod segment;
mod vents;

static DAYS: [fn(); 8] = [
    || {
        let input = scanner::get_input();
        println!("1-1: {}", scanner::get_increment_count(&input));
        println!("1-2: {}", scanner::get_increment_window_count(&input, 3));
    },
    || {
        let input = navigation::get_input();
        println!("2-1: {:?}", navigation::get_bearings_old(&input));
        println!("2-2: {:?}", navigation::get_bearings(&input));
    },
    || {
        let input = diagnostic::get_input();
        println!("3-1: {}", diagnostic::get_power_consumption(&input));
        println!("3-2: {}", diagnostic::get_life_support_rating(&input));
    },
    || {
        let numbers = bingo::get_numbers();
        let cards = bingo::get_cards();
        println!("4-1: {}", bingo::find_winner(&cards, &numbers));
        println!("4-2: {}", bingo::find_last_winner(&cards, &numbers));
    },
    || {
        let input = vents::get_input();
        let map_1 = vents::generate_map(&input, false);
        let map_2 = vents::generate_map(&input, true);
        println!("5-1: {}", vents::elevation_count(&map_1, 2));
        println!("5-2: {}", vents::elevation_count(&map_2, 2));
    },
    || {
        let numbers = lanternfish::get_numbers();
        println!("6-1: {}", lanternfish::calculate_growth(&numbers, 80));
        println!("6-2: {}", lanternfish::calculate_growth(&numbers, 256));
    },
    || {
        let positions = crabs::get_numbers();
        println!("7-1: {}", crabs::get_position(&positions, true));
        println!("7-2: {}", crabs::get_position(&positions, false));
    },
    || {
        let input = segment::get_input();
        println!("8-1: {}", segment::count_unique_digits(&input));
        //println!("8-2: {}", crabs::get_position(&positions, false));
    },
];

pub fn all() {
    ln();
    for day in DAYS.iter() {
        day();
        ln();
    }
}

pub fn day(day: usize) {
    if day <= DAYS.len() {
        ln();
        DAYS[day - 1]();
        ln();
    } else {
        println!("Day {} not implemented", day);
    }
}

fn ln() {
    println!("----------");
}
