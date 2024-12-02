use std::env;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_index = &args[1].parse::<i32>().expect("Invalid day passed.");

    match day_index {
        1 => day1::solver::solve(),
        2 => day2::solver::solve(),
        _ => println!("No day passed. Quitting."),
    }
}
