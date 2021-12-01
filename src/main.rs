use structopt::StructOpt;
mod lib;

#[derive(StructOpt, Debug)]
enum Cli {
    #[structopt(about = "Runs all days")]
    All,
    #[structopt(about = "Runs a specified day")]
    Day {
        #[structopt(name = "day to run")]
        day: u8,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::All {} => all(),
        Cli::Day { day } => match day {
            1 => day_one(),
            _ => println!("Day {} not found", day),
        },
    }
}

fn all() {
    day_one()
}

fn day_one() {
    let input = lib::get_input("day-1");
    println!("1-1: {}", lib::get_increment_count(&input.real));
    println!("1-2: {}", lib::get_increment_window_count(&input.real, 3));
}
