use std::fs;

fn parse_horizontals(input: &Vec<Vec<char>>) -> i32 {
    let mut ct = 0;
    for line in input {
        for i in 0..line.len() - 4 {
            if (line[i] == 'X' && line[i + 1] == 'M' && line[i + 2] == 'A' && line[i + 3] == 'S')
                || (line[i] == 'S'
                    && line[i + 1] == 'A'
                    && line[i + 2] == 'M'
                    && line[i + 3] == 'X')
            {
                ct += 1;
            }
        }
    }
    ct
}

fn parse_diagonals(input: &Vec<Vec<char>>) -> i32 {
    let mut ct = 0;
    for d in 3..input.len() {
        let mut x = d;
        let mut y = 0;
        while x > 2 && y > 2 {
            if (input[d][y] == 'X'
                && input[d - 1][y - 1] == 'M'
                && input[d - 2][y - 2] == 'A'
                && input[d - 3][y - 3] == 'S')
                || (input[d][y] == 'S'
                    && input[d - 1][y - 1] == 'A'
                    && input[d - 2][y - 2] == 'M'
                    && input[d - 3][y - 3] == 'X')
            {
                ct += 1;
            }
            x -= 1;
            y -= 1;
        }
        
        x = d;
        y = 0;
        while x < input[d].len() - 3 && y < input.len() - 3 {
            if (input[d][y] == 'X'
                && input[d + 1][y + 1] == 'M'
                && input[d + 2][y + 2] == 'A'
                && input[d + 3][y + 3] == 'S')
                || (input[d][y] == 'S'
                    && input[d + 1][y + 1] == 'A'
                    && input[d + 2][y + 2] == 'M'
                    && input[d + 3][y + 3] == 'X')
            {
                ct += 1;
            }
            x += 1;
            y += 1;
        }
    }
    ct
}

fn count_xmas(input: &Vec<Vec<char>>) -> i32 {
    parse_diagonals(input) + parse_horizontals(input)
}

fn load_data(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve() {
    let test_path = "src/day4/test.txt";
    let test_input_str = fs::read_to_string(test_path).expect("File should exist");

    let test_input = load_data(&test_input_str);

    let test_result = count_xmas(&test_input);
    println!("Test result: {}", test_result);
}
