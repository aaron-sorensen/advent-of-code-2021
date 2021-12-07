mod bingo;
mod diagnostic;
mod navigation;
mod scanner;
mod vents;

pub fn all() {
    day_one();
    println!("-");
    day_two();
    println!("-");
    day_three();
    println!("-");
    day_four();
    println!("-");
    day_five();
}

pub fn day_one() {
    let input = scanner::get_input("day-1");
    println!("1-1: {}", scanner::get_increment_count(&input.real));
    println!(
        "1-2: {}",
        scanner::get_increment_window_count(&input.real, 3)
    );
}

pub fn day_two() {
    let input = navigation::get_input("day-2");
    println!("2-1: {:?}", navigation::get_bearings_old(&input));
    println!("2-2: {:?}", navigation::get_bearings(&input));
}

pub fn day_three() {
    let input = diagnostic::get_input("day-3");
    println!("3-1: {}", diagnostic::get_power_consumption(&input));
    println!("3-2: {}", diagnostic::get_life_support_rating(&input));
}

pub fn day_four() {
    let numbers = bingo::get_numbers("day-4");
    let cards = bingo::get_cards("day-4");
    println!("4-1: {}", bingo::find_winner(&cards, &numbers));
    println!("4-2: {}", bingo::find_last_winner(&cards, &numbers));
}

pub fn day_five() {
    let input = vents::get_input("day-5");
    let map_1 = vents::generate_map(&input, false);
    let map_2 = vents::generate_map(&input, true);
    println!("5-1: {}", vents::elevation_count(&map_1, 2));
    println!("5-2: {}", vents::elevation_count(&map_2, 2));
}
