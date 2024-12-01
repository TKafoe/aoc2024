fn read_input(input_str: &str) -> (Vec<i32>, Vec<i32>) {
    let split = input_str.lines();

    let mut l = Vec::new();
    let mut r = Vec::new();
    for n in split {
        let mut split_on_whitespace = n.split_whitespace();
        l.push(
            split_on_whitespace
                .next()
                .expect("This line should exist")
                .parse::<i32>()
                .expect("Input should be a number"),
        );
        r.push(
            split_on_whitespace
                .next()
                .expect("This line should exist")
                .parse::<i32>()
                .expect("Input should be a number"),
        );
    }

    (l, r)
}

fn solve_part_1(mut l: Vec<i32>, mut r: Vec<i32>) -> i32 {
    l.sort();
    r.sort();

    let mut sum = 0;
    for i in 0..l.len() {
        sum += (l[i] - r[i]).abs();
    }

    sum
}

fn solve_part_2(l: Vec<i32>, r: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..l.len() {
        sum += l[i] * r.iter().filter(|&n| *n == l[i]).count() as i32;
    }

    sum
}

fn solve_file(file_path: &str) -> (i32, i32) {
    let file_content =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file.");

    let (l, r) = read_input(&file_content);

    (
        solve_part_1(l.clone(), r.clone()),
        solve_part_2(l.clone(), r.clone()),
    )
}

pub fn solve() {
    let test_file = "src/day1/test.txt";
    let input_file = "src/day1/input.txt";

    let (result_test_p1, result_test_p2) = solve_file(&test_file);
    let (result_input_p1, result_input_p2) = solve_file(&input_file);

    println!("p1: {}, p2: {}", result_test_p1, result_test_p2);
    println!("p1: {}, p2: {}", result_input_p1, result_input_p2);
}
