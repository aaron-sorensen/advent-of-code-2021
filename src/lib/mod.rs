mod diagnostic;
mod navigation;
mod scanner;

pub fn all() {
    day_one();
    day_two();
    day_three();
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
