use regex::Regex;
use std::fs;

fn extract_muls_without_disables(input: &str) -> Vec<(i32, i32)> {
    let re_pattern = r"(mul\((\d+),(\d+)\))";

    extract_muls(input, re_pattern)
}

fn extract_muls_with_disables(input: &str) -> Vec<(i32, i32)> {
    let re_pattern = r"(do\(\)()())|(don't\(\)()())|(mul\((\d+),(\d+)\))";

    extract_muls(input, &re_pattern)
}

fn extract_muls(input: &str, re_pattern: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(re_pattern).unwrap();
    let mut muls = Vec::new();

    let mut enabled = true;
    for (_, [group1, group2, group3]) in re.captures_iter(input).map(|c| c.extract()) {
        if group1 == "do()" {
            enabled = true;
        } else if group1 == "don't()" {
            enabled = false;
        } else if enabled {
            muls.push((
                group2.parse::<i32>().unwrap(),
                group3.parse::<i32>().unwrap(),
            ));
        }
    }

    muls
}

fn multiply(muls: Vec<(i32, i32)>) -> i32 {
    muls.iter().fold(0, |acc, (num1, num2)| acc + num1 * num2)
}

fn multiply_uncorrupted_memory(input: &str, use_disables: bool) -> i32 {
    let muls;
    if !use_disables {
        muls = extract_muls_without_disables(input);
    } else {
        muls = extract_muls_with_disables(input);
    }

    let result = multiply(muls);

    result
}

pub fn solve() {
    let test_input_path = "src/day3/test.txt";
    let test_corrupted_input_path = "src/day3/test2.txt";
    let input_path = "src/day3/input.txt";

    let test_file_str = fs::read_to_string(test_input_path).expect("Test file should exist.");
    let test_corrupted_file_str =
        fs::read_to_string(test_corrupted_input_path).expect("Test2 file should exist.");
    let file_str = fs::read_to_string(input_path).expect("File should exist.");

    let test_result = multiply_uncorrupted_memory(&test_file_str, false);
    let result = multiply_uncorrupted_memory(&file_str, false);

    println!("Test result p1: {}", test_result);
    println!("Result p1: {}", result);

    let file_str2 = fs::read_to_string(input_path).expect("File should exist.");
    let test_corrupted_result = multiply_uncorrupted_memory(&test_corrupted_file_str, true);
    let result_corrupted = multiply_uncorrupted_memory(&file_str2, true);

    println!("Test result p2: {}", test_corrupted_result);
    println!("Result p2: {}", result_corrupted);
}
