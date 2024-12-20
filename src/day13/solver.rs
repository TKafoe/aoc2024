use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};
use regex::{self, Regex};
use std::fs;

fn load_data(path: &str) -> Vec<Vec<(i64, i64)>> {
    let data = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let d_regex = r"\d+";

    let mut result = Vec::new();
    let mut pt = 0;
    let re = Regex::new(d_regex).unwrap();
    while pt < lines.len() {
        let mut problem = Vec::new();
        for _ in 0..3 {
            let mut captures = re.captures_iter(lines[pt]);
            problem.push((
                captures.next().unwrap()[0].parse::<i64>().unwrap(),
                captures.next().unwrap()[0].parse::<i64>().unwrap(),
            ));
            pt += 1;
        }
        result.push(problem);

        pt += 1;
    }

    result
}

fn get_solutions(problem: &Vec<(i64, i64)>, corrected: bool) -> (BigInt, BigInt) {
    let x1 = BigRational::from_integer(BigInt::from(problem[0].0));
    let x2 = BigRational::from_integer(BigInt::from(problem[1].0));
    let mut x3 = BigRational::from_integer(BigInt::from(problem[2].0));
    let y1 = BigRational::from_integer(BigInt::from(problem[0].1));
    let y2 = BigRational::from_integer(BigInt::from(problem[1].1));
    let mut y3 = BigRational::from_integer(BigInt::from(problem[2].1));

    if corrected {
        x3 += BigInt::from(10000000000000 as i128);
        y3 += BigInt::from(10000000000000 as i128);
    }

    let a = (((&x2 * &y3) / &y2) - &x3) / (((&y1 * &x2) / &y2) - &x1);
    let b = (&x3 - &a * &x1) / &x2;

    (a.to_integer(), b.to_integer())
}

fn cost(problem: &Vec<(i64, i64)>, corrected: bool) -> BigInt {
    let (a, b) = get_solutions(problem, corrected);

    if !corrected
        && (a.gt(&BigInt::from(100))
            || b.gt(&BigInt::from(100))
            || a.lt(&BigInt::from(0))
            || b.lt(&BigInt::from(0)))
    {
        return BigInt::from(0);
    }

    if !corrected
        && (!(&a * problem[0].0 + &b * problem[1].0 == BigInt::from(problem[2].0))
            || !(&a * problem[0].1 + &b * problem[1].1 == BigInt::from(problem[2].1)))
    {
        return BigInt::from(0);
    }

    if corrected
        && (!(&a * problem[0].0 + &b * problem[1].0
            == BigInt::from(problem[2].0) + BigInt::from(10000000000000 as i128))
            || !(&a * problem[0].1 + &b * problem[1].1
                == BigInt::from(problem[2].1) + BigInt::from(10000000000000 as i128)))
    {
        return BigInt::from(0);
    }

    3 * a + b
}

fn compute_costs(problems: &Vec<Vec<(i64, i64)>>, corrected: bool) -> BigInt {
    problems.iter().fold(BigInt::from(0), |acc, problem| {
        acc + cost(problem, corrected)
    })
}

pub fn solve() {
    let test_path = "src/day13/test.txt";
    let test_data = load_data(test_path);
    let test_result = compute_costs(&test_data, false);
    let test_result_p2 = compute_costs(&test_data, true);

    println!("Test result: {}", test_result);
    println!("Test result p2: {}", test_result_p2);

    let path = "src/day13/input.txt";
    let data = load_data(path);
    let result = compute_costs(&data, false);
    let result_p2 = compute_costs(&data, true);

    println!("Result: {}", result);
    println!("Result p2: {}", result_p2);
}
