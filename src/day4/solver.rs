use std::fs;

fn parse_horizontals(input: &Vec<Vec<(char, (i32, i32))>>, m: &mut Vec<Vec<char>>) -> i32 {
    let mut ct = 0;
    for j in 0..input.len() {
        let line = &input[j];
        if line.len() < 4 {
            continue;
        }
        for i in 0..line.len() - 3 {
            if (line[i].0 == 'X'
                && line[i + 1].0 == 'M'
                && line[i + 2].0 == 'A'
                && line[i + 3].0 == 'S')
                || (line[i].0 == 'S'
                    && line[i + 1].0 == 'A'
                    && line[i + 2].0 == 'M'
                    && line[i + 3].0 == 'X')
            {
                ct += 1;
                m[line[i].1 .0 as usize][line[i].1 .1 as usize] = line[i].0;
                m[line[i + 1].1 .0 as usize][line[i + 1].1 .1 as usize] = line[i + 1].0;
                m[line[i + 2].1 .0 as usize][line[i + 2].1 .1 as usize] = line[i + 2].0;
                m[line[i + 3].1 .0 as usize][line[i + 3].1 .1 as usize] = line[i + 3].0;
            }
        }
    }
    ct
}

fn extract_verticals(input: &Vec<Vec<(char, (i32, i32))>>) -> Vec<Vec<(char, (i32, i32))>> {
    let mut verticals = Vec::new();
    for c in 0..input[0].len() {
        let mut vertical = Vec::new();

        for r in 0..input.len() {
            vertical.push(input[r][c].clone());
        }

        verticals.push(vertical);
    }

    verticals
}

fn extract_diagonals_down(input: &Vec<Vec<(char, (i32, i32))>>) -> Vec<Vec<(char, (i32, i32))>> {
    let mut diagonals = Vec::new();

    for r_i in 0..input.len() {
        let mut diagonal = Vec::new();
        let mut t_r_i = r_i;
        let mut c_i = 0;

        while t_r_i < input.len() && c_i < input[r_i].len() {
            diagonal.push(input[t_r_i][c_i].clone());

            t_r_i += 1;
            c_i += 1;
        }

        diagonals.push(diagonal);
    }

    for c_i in 1..input[0].len() {
        let mut diagonal = Vec::new();
        let mut t_c_i = c_i;
        let mut r_i = 0;

        while t_c_i < input[r_i].len() && r_i < input.len() {
            diagonal.push(input[r_i][t_c_i].clone());

            r_i += 1;
            t_c_i += 1;
        }

        diagonals.push(diagonal);
    }

    diagonals
}

fn extract_diagonals_up(input: &Vec<Vec<(char, (i32, i32))>>) -> Vec<Vec<(char, (i32, i32))>> {
    let mut diagonals = Vec::new();
    for r_i in 0..input.len() {
        let mut diagonal = Vec::new();
        let mut t_r_i = r_i as i32;
        let mut c_i = 0;

        while t_r_i >= 0 && c_i < input[r_i].len() {
            diagonal.push(input[t_r_i as usize][c_i].clone());

            t_r_i -= 1;
            c_i += 1;
        }

        diagonals.push(diagonal);
    }

    for c_i in 1..input[input.len() - 1].len() {
        let mut diagonal = Vec::new();
        let mut t_c_i = c_i;
        let mut r_i = (input.len() - 1) as i32;

        while t_c_i < input[input.len() - 1].len() && r_i >= 0 {
            diagonal.push(input[r_i as usize][t_c_i].clone());

            r_i -= 1;
            t_c_i += 1;
        }

        diagonals.push(diagonal);
    }

    diagonals
}

fn parse_diagonals(input: &Vec<Vec<(char, (i32, i32))>>, m: &mut Vec<Vec<char>>) -> i32 {
    let diagonal_set_1 = extract_diagonals_up(input);
    let diagonal_set_2 = extract_diagonals_down(input);

    parse_horizontals(&diagonal_set_1, m) + parse_horizontals(&diagonal_set_2, m)
}

fn parse_verticals(input: &Vec<Vec<(char, (i32, i32))>>, m: &mut Vec<Vec<char>>) -> i32 {
    let verticals = extract_verticals(input);

    parse_horizontals(&verticals, m)
}

fn count_xmas(input: &Vec<Vec<(char, (i32, i32))>>) -> (i32, i32) {
    let mut m = Vec::new();
    for row in input {
        let mut char_v = Vec::new();
        for _ in row {
            char_v.push('.');
        }
        m.push(char_v);
    }

    let result = parse_diagonals(input, &mut m)
        + parse_horizontals(input, &mut m)
        + parse_verticals(input, &mut m);

    let result_2 = count_x_mas(input);

    (result, result_2)
}

fn check_mas(c1: char, c3: char) -> bool {
    if c1 == 'M' && c3 == 'S' {
        return true;
    }
    if c1 == 'S' && c3 == 'M' {
        return true;
    }
    false
}

fn count_x_mas(m: &Vec<Vec<(char, (i32, i32))>>) -> i32 {
    let mut ct = 0;
    for r in 1..m.len() - 1 {
        for c in 1..m[r].len() - 1 {
            if m[r][c].0 == 'A' {
                if check_mas(m[r - 1][c - 1].0, m[r + 1][c + 1].0)
                    && check_mas(m[r + 1][c - 1].0, m[r - 1][c + 1].0)
                {
                    ct += 1;
                }
            }
        }
    }
    ct
}

fn load_data(input_str: &str) -> Vec<Vec<(char, (i32, i32))>> {
    let input: Vec<Vec<char>> = input_str.lines().map(|c| c.chars().collect()).collect();

    let mut data = Vec::new();
    for i in 0..input.len() {
        let mut char_line: Vec<(char, (i32, i32))> = Vec::new();
        for c in 0..input[i].len() {
            char_line.push((input[i][c], (i as i32, c as i32)));
        }
        data.push(char_line);
    }

    data
}

pub fn solve() {
    let test_path = "src/day4/test.txt";
    let input_path = "src/day4/input.txt";
    let test_input_str = fs::read_to_string(test_path).expect("File should exist");
    let input_str = fs::read_to_string(input_path).expect("File should exist");

    let test_input = load_data(&test_input_str);
    let input = load_data(&input_str);

    let (test_result_1, test_result_2) = count_xmas(&test_input);
    let (result_1, result_2) = count_xmas(&input);
    println!("Test result 1: {}", test_result_1);
    println!("Input result 1: {}", result_1);
    println!("Test result 2: {}", test_result_2);
    println!("Input result 2: {}", result_2);
}
