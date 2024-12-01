use std::env;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_index = &args[1].parse::<i32>().expect("Invalid day passed.");

    match day_index {
        1 => day1::solver::solve(),
        _ => println!("No day passed. Quitting."),
    }
}
