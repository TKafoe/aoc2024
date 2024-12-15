use std::fs;

fn load_data(path: &str) -> Vec<String> {
    let data: Vec<char> = fs::read_to_string(path)
        .expect("File should exist")
        .chars()
        .collect();

    let mut parsed_data: Vec<String> = Vec::new();
    let mut flip = false;
    let mut ct: u32 = 0;
    for i in 0..data.len() - 1 {
        let c = data[i];
        let d = c.to_digit(10).expect("Should be an integer");
        for _ in 0..d {
            if flip {
                parsed_data.push(".".to_string());
            } else {
                parsed_data.push(ct.to_string());
            }
        }

        if !flip {
            ct += 1;
        }
        flip = !flip;
    }

    parsed_data
}

fn rearrange(input: &mut Vec<String>) {
    let mut pt1 = 0;
    let mut pt2 = input.len() - 1;

    while pt1 != pt2 {
        if input[pt2] == "." {
            pt2 -= 1;
            continue;
        }

        if input[pt1] == "." {
            input[pt1] = input[pt2].clone();
            input[pt2] = ".".to_string();
            pt1 += 1;
            pt2 -= 1;
        } else {
            pt1 += 1;
        }
    }
}

fn find_free_space(input: &Vec<String>, len: i32, i: i32) -> i32 {
    let mut pt1: usize = 0;
    while pt1 < (i - len) as usize {
        if input[pt1] != "." {
            pt1 += 1;
            continue;
        }

        let mut ct = 0;
        let t_pt1 = pt1;
        while input[pt1] == "." {
            pt1 += 1;
            ct += 1;
        }

        if ct >= len {
            return t_pt1 as i32;
        }
    }

    -1
}

fn rearrange_p2(input: &mut Vec<String>) {
    let mut pt2 = input.len() - 1;
    while pt2 != 0 {
        while input[pt2] == "." {
            pt2 -= 1;
        }

        let mut ct_r = 0;
        let mut tpt2 = pt2;
        let t = input[tpt2].clone();
        while input[tpt2] == t {
            ct_r += 1;
            tpt2 -= 1;

            if tpt2 == 0 {
                return;
            }
        }
        
        // I think I can improve this by storing the free spaces
        // before going over them, but I can not be bothered.
        let free_index = find_free_space(input, ct_r, pt2 as i32);

        if free_index >= 0 {
            for i in 0..ct_r {
                input[(free_index + i) as usize] = t.clone();
                input[pt2 - i as usize] = ".".to_string();
            }
        }
        pt2 -= ct_r as usize;
    }
}

fn compute_checksum(input: &Vec<String>) -> i128 {
    let mut sum = 0;
    for i in 0..input.len() {
        if input[i] == "." {
            continue;
        }
        sum += i as i128 * input[i].parse::<i128>().expect("Should be number");
    }

    sum
}

pub fn solve() {
    let test_path = "src/day9/test.txt";
    let mut test_input = load_data(&test_path);

    let path = "src/day9/input.txt";
    let mut input = load_data(&path);

    rearrange(&mut test_input);
    rearrange(&mut input);

    let test_result = compute_checksum(&test_input);
    let result = compute_checksum(&input);
    println!("Test result: {}", test_result);
    println!("Result: {}", result);

    test_input = load_data(&test_path);
    input = load_data(&path);
    
    rearrange_p2(&mut test_input);
    rearrange_p2(&mut input);
    
    let test_result_2 = compute_checksum(&test_input);
    let result_2 = compute_checksum(&input);

    println!("Test result 2: {}", test_result_2);
    println!("Result 2: {}", result_2);
}
