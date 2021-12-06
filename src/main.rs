use structopt::StructOpt;
pub mod lib;

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
        Cli::All {} => lib::all(),
        Cli::Day { day } => match day {
            1 => lib::day_one(),
            2 => lib::day_two(),
            3 => lib::day_three(),
            4 => lib::day_four(),
            _ => println!("Day {} not found", day),
        },
    }
}
