use std::fs;

/// Parse report into a vector of integers.
///
/// # Example
///
/// ```
/// let result = parse_report("1 2 3");
/// assert_eq!(result, [1, 2, 3]);
/// ```
fn parse_report(report_str: &str) -> Vec<i32> {
    let mut level: Vec<i32> = Vec::new();
    report_str.split_whitespace().for_each(|number| {
        let number: i32 = number
            .parse::<i32>()
            .expect("Wrong input. Each level should hold numbers.");

        level.push(number);
    });

    level
}

/// Parse input string into a vector of vectors of integers.
///
/// # Example
///
/// ```
/// let result = parse_input("1 2 3\n2 3 4\n");
/// assert_eq!(result, [[1, 2, 3], [2, 3, 4]]);
/// ```
fn parse_input(input_str: &str) -> Vec<Vec<i32>> {
    let lines = input_str.lines();

    let mut reports: Vec<Vec<i32>> = Vec::new();
    lines.for_each(|f| {
        let level = parse_report(f);
        reports.push(level);
    });

    reports
}

/// Checks if two consecutive levels are unsafe.
///
/// # Arguments
/// * `level1` - The first level
/// * `level2` - The second level
/// * `decreasing` - Whether the comparison should be considering
///                  the numbers as decreasing or increasing.
///
/// # Example
///
/// ```
/// let result = is_correct_level(2, 3, false);
/// assert_eq!(result, true);
///
/// let result = is_correct_level(2, 3, true);
/// assert_eq!(result, false);
/// ```
fn is_correct_level(level1: i32, level2: i32, decreasing: bool) -> bool {
    if level1 == level2 {
        return false;
    }

    if decreasing {
        if level1 < level2 {
            return false;
        }
        let diff = level1 - level2;
        if diff == 0 || diff > 3 {
            return false;
        }
        return true;
    } else {
        if level2 < level1 {
            return false;
        }
        let diff = level2 - level1;
        if diff == 0 || diff > 3 {
            return false;
        }

        return true;
    }
}

/// Checks if a report is considered safe. You can
/// optionally use the problem dampener.
///
/// # Example
///
/// ```
/// let result = is_safe([1, 2, 3], false);
/// assert_eq!(result, true);
///
/// let result = is_safe([2, 1, 3], false);
/// assert_eq!(result, false);
///
/// let result = is_safe([1, 2, 9, 3], true);
/// assert_eq!(result, true);
/// ```
fn is_safe(report: &Vec<i32>, use_problem_dampener: bool) -> bool {
    let mut decreasing = true;
    if report[0] < report[1] {
        decreasing = false;
    }

    let mut safe = true;
    for i in 1..report.len() {
        safe = is_correct_level(report[i - 1], report[i], decreasing);
        if !safe {
            break;
        }
    }

    if !use_problem_dampener || safe {
        return safe;
    }

    for i in 0..report.len() {
        let mut changed_report = report.to_vec();
        changed_report.remove(i);
        if is_safe(&changed_report, false) {
            return true;
        }
    }

    false
}

/// Check all reports from the input on safety. You can
/// optionally use the problem dampener.
///
/// # Example
///
/// ```
/// let result = check_reports([
///     [1, 2, 3], [4, 5, 6]
/// ], false);
/// assert_eq!(result, true);
///
/// let result = check_reports([
///     [1, 9, 3], [4, 5, 6]
/// ], false);
/// assert_eq!(result, false);
///
/// let result = check_reports([
///     [1, 9, 3], [4, 5, 6]
/// ], true);
/// assert_eq!(result, true);
/// ```
fn check_reports(input: &Vec<Vec<i32>>, use_problem_dampener: bool) -> i32 {
    let mut sum = 0;
    for report in input {
        if is_safe(report, use_problem_dampener) {
            sum += 1;
        }
    }

    sum
}

pub fn solve() {
    let test_input_path = "src/day2/test.txt";
    let input_path = "src/day2/input.txt";

    let test_file_str = fs::read_to_string(test_input_path).expect("Test file should exist.");
    let input_file_str = fs::read_to_string(input_path).expect("Input file should exist.");

    let test_input = parse_input(&test_file_str);
    let input = parse_input(&input_file_str);

    let test_result_p1 = check_reports(&test_input, false);
    let input_result_p1 = check_reports(&input, false);

    println!("P1 Test input: {} levels are safe", test_result_p1);
    println!("P1 Input: {} levels are safe", input_result_p1);

    let test_result_p2 = check_reports(&test_input, true);
    let input_result_p2 = check_reports(&input, true);

    println!("P2 Test input: {} levels are safe", test_result_p2);
    println!("P2 Input: {} levels are safe", input_result_p2);
}
