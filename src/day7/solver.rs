use std::fs;

fn solve_equation(mut num: i128, terms: &[i128], target: i128, use_third_operator: bool) -> bool {
    if terms.len() == 0 && num == target {
        return true;
    }

    if terms.len() == 0 {
        return false;
    }

    if num > target {
        return false;
    }

    num *= terms[0];

    if solve_equation(num, &terms[1..], target, use_third_operator) {
        return true;
    }

    num /= terms[0];
    num += terms[0];
    if solve_equation(num, &terms[1..], target, use_third_operator) {
        return true;
    }

    if use_third_operator {
        num -= terms[0];
        let new_num = format!("{}{}", num, terms[0])
            .parse::<i128>()
            .expect("Should be a number");
        if solve_equation(new_num, &terms[1..], target, use_third_operator) {
            return true;
        }
    }

    false
}

fn load_data(path: &str) -> Vec<(i128, Vec<i128>)> {
    let data = fs::read_to_string(path).expect("Should exist");
    let lines = data.lines();

    let mut equations: Vec<(i128, Vec<i128>)> = Vec::new();
    for line in lines {
        let mut split = line.split(": ");
        let eq_solution = split
            .next()
            .expect("Should exist")
            .parse::<i128>()
            .expect("Should be an integer");
        let terms_split = split
            .next()
            .expect("Should exist")
            .split(" ")
            .map(|s| s.parse::<i128>().expect("Should be an integer"))
            .collect();

        equations.push((eq_solution, terms_split));
    }

    equations
}

fn solve_equations(equations: &Vec<(i128, Vec<i128>)>, use_third_operator: bool) -> i128 {
    let mut sum = 0;
    for equation in equations {
        if solve_equation(
            equation.1[0],
            &equation.1[1..equation.1.len()],
            equation.0,
            use_third_operator,
        ) {
            sum += equation.0;
        }
    }
    sum
}

pub fn solve() {
    let test_path = "src/day7/test.txt";
    let test_data = load_data(test_path);
    let test_result = solve_equations(&test_data, false);
    let test_result_2 = solve_equations(&test_data, true);

    let path = "src/day7/input.txt";
    let data = load_data(path);
    let result = solve_equations(&data, false);
    let result_2 = solve_equations(&data, true);

    println!("Test result: {}", test_result);
    println!("Test result 2: {}", test_result_2);
    println!("Result: {}", result);
    println!("Result 2: {}", result_2);
}
