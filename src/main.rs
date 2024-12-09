use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_index = &args[1].parse::<i32>().expect("Invalid day passed.");

    match day_index {
        1 => day1::solver::solve(),
        2 => day2::solver::solve(),
        3 => day3::solver::solve(),
        4 => day4::solver::solve(),
        5 => day5::solver::solve(),
        6 => day6::solver::solve(),
        7 => day7::solver::solve(),
        _ => println!("No day passed. Quitting."),
    }
}
