use structopt::StructOpt;
pub mod lib;

#[derive(StructOpt, Debug)]
enum Cli {
    #[structopt(about = "Runs all days")]
    All,
    #[structopt(about = "Runs a specified day")]
    Day {
        #[structopt(name = "day to run")]
        day: usize,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::All {} => lib::all(),
        Cli::Day { day } => lib::day(day),
    }
}
