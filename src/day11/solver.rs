use std::{collections::HashMap, fs};

fn load_data(path: &str) -> Vec<usize> {
    let str_data = fs::read_to_string(path).expect("Should exist");

    let split_data = str_data.split(" ");

    let mut res = Vec::new();
    for split in split_data {
        res.push(split.trim_end().parse::<usize>().expect("Should be int"));
    }

    res
}

fn blink_recursively(num: usize, level: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if mem.contains_key(&(num, level)) {
        return *mem.get(&(num, level)).expect("Key should exist.");
    }

    if level == 0 {
        return 1;
    }

    let mut sum = 0;
    if num == 0 {
        sum += blink_recursively(1, level - 1, mem);
    } else if num.to_string().len() % 2 == 0 {
        let str_val = num.to_string();
        let num1 = str_val[..(str_val.len() / 2)]
            .parse::<usize>()
            .expect("Should be num");
        let num2 = str_val[(str_val.len() / 2)..]
            .parse::<usize>()
            .expect("Should be num");
        sum += blink_recursively(num1, level - 1, mem);
        sum += blink_recursively(num2, level - 1, mem);
    } else {
        sum += blink_recursively(num * 2024, level - 1, mem);
    }
    
    mem.insert((num, level), sum);

    sum
}

fn blink_n_recursively(n: usize, input: &Vec<usize>) -> usize {
    let mut sum = 0;
    let mut mem = HashMap::new();
    for i in 0..input.len() {
        sum += blink_recursively(input[i], n, &mut mem);
    }
    sum
}

pub fn solve() {
    let test_path = "src/day11/test.txt";
    let test_data = load_data(&test_path);
    let test_result_p1 = blink_n_recursively(25, &test_data);
    let test_result_p2 = blink_n_recursively(75, &test_data);

    println!("Test result p1: {}", test_result_p1);
    println!("Test result p2: {}", test_result_p2);

    let path = "src/day11/input.txt";
    let data = load_data(&path);
    let result_p1 = blink_n_recursively(25, &data);
    let result_p2 = blink_n_recursively(75, &data);

    println!("Result p1: {}", result_p1);
    println!("Result p2: {}", result_p2);
}
