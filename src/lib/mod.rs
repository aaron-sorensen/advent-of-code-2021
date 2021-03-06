mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

static DAYS: [fn(); 14] = [
    || {
        println!("1-1: {}", day_1::part_1());
        println!("1-2: {}", day_1::part_2());
    },
    || {
        println!("2-1: {:?}", day_2::part_1());
        println!("2-2: {:?}", day_2::part_2());
    },
    || {
        println!("3-1: {}", day_3::part_1());
        println!("3-2: {}", day_3::part_2());
    },
    || {
        println!("4-1: {}", day_4::part_1());
        println!("4-2: {}", day_4::part_2());
    },
    || {
        println!("5-1: {}", day_5::part_1());
        println!("5-2: {}", day_5::part_2());
    },
    || {
        println!("6-1: {}", day_6::part_1());
        println!("6-2: {}", day_6::part_2());
    },
    || {
        println!("7-1: {}", day_7::part_1());
        println!("7-2: {}", day_7::part_2());
    },
    || {
        println!("8-1: {}", day_8::part_1());
        println!("8-2: {}", day_8::part_2());
    },
    || {
        println!("9-1: {}", day_9::part_1());
        println!("9-2: {}", day_9::part_2());
    },
    || {
        println!("10-1: {}", day_10::part_1());
        println!("10-2: {}", day_10::part_2());
    },
    || {
        println!("11-1: {}", day_11::part_1());
        println!("11-2: {}", day_11::part_2());
    },
    || {
        println!("12-1: {}", day_12::part_1());
        println!("12-2: {}", day_12::part_2());
    },
    || {
        println!("13-1: {}", day_13::part_1());
        println!("13-2:");
        day_13::part_2();
    },
    || {
        println!("14-1: {}", day_14::part_1());
        println!("14-2: {}", day_14::part_2());
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
