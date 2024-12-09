use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_data(path: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let input_str = fs::read_to_string(path).expect("File should exist");
    let field: Vec<Vec<char>> = input_str.lines().map(|c| c.chars().collect()).collect();

    let mut start_pos = (0, 0);
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == '^' {
                start_pos = (i as i32, j as i32);
            }
        }
    }
    assert!(start_pos != (0, 0));

    (field, start_pos)
}

fn move_guard(pos: (i32, i32), field: &mut Vec<Vec<char>>) -> (i32, i32) {
    let direction = field[pos.0 as usize][pos.1 as usize];
    assert!(direction == '^' || direction == '>' || direction == '<' || direction == 'v');

    match direction {
        '^' => {
            if pos.0 - 1 < 0 {
                return (pos.0 - 1, pos.1);
            }

            if field[(pos.0 - 1) as usize][pos.1 as usize] == '#' {
                field[pos.0 as usize][pos.1 as usize] = '>';
                return (pos.0, pos.1);
            }
            field[(pos.0) as usize][pos.1 as usize] = '.';
            field[(pos.0 - 1) as usize][pos.1 as usize] = '^';
            return (pos.0 - 1, pos.1);
        }
        '>' => {
            if pos.1 + 1 >= field.len() as i32 {
                return (pos.0, pos.1 + 1);
            }

            if field[(pos.0) as usize][(pos.1 + 1) as usize] == '#' {
                field[pos.0 as usize][pos.1 as usize] = 'v';
                return (pos.0, pos.1);
            }
            field[(pos.0) as usize][pos.1 as usize] = '.';
            field[pos.0 as usize][(pos.1 + 1) as usize] = '>';
            return (pos.0, pos.1 + 1);
        }
        '<' => {
            if pos.1 - 1 < 0 {
                return (pos.0, pos.1 - 1);
            }

            if field[pos.0 as usize][(pos.1 - 1) as usize] == '#' {
                field[pos.0 as usize][pos.1 as usize] = '^';
                return (pos.0, pos.1);
            }
            field[(pos.0) as usize][pos.1 as usize] = '.';
            field[pos.0 as usize][(pos.1 - 1) as usize] = '<';
            return (pos.0, pos.1 - 1);
        }
        'v' => {
            if pos.0 + 1 >= field[0].len() as i32 {
                return (pos.0 + 1, pos.1);
            }

            if field[(pos.0 + 1) as usize][pos.1 as usize] == '#' {
                field[pos.0 as usize][pos.1 as usize] = '<';
                return (pos.0, pos.1);
            }
            field[(pos.0) as usize][pos.1 as usize] = '.';
            field[(pos.0 + 1) as usize][pos.1 as usize] = 'v';
            return (pos.0 + 1, pos.1);
        }
        _ => assert!(false),
    }
    assert!(false);
    (0, 0)
}

fn move_guard_until_off_map(
    start_pos: (i32, i32),
    field: &mut Vec<Vec<char>>,
) -> (usize, HashSet<(i32, i32)>) {
    let mut seen = HashSet::new();
    seen.insert(start_pos);
    let mut new_pos = move_guard(start_pos, field);
    while 0 <= new_pos.0
        && new_pos.0 < field.len() as i32
        && 0 <= new_pos.1
        && new_pos.1 < field[0].len() as i32
    {
        seen.insert(new_pos);
        new_pos = move_guard(new_pos, field);
    }

    seen.remove(&start_pos);
    (seen.len(), seen)
}

fn move_guard_until_loop_or_off_map(start_pos: (i32, i32), field: &mut Vec<Vec<char>>) -> bool {
    let mut seen = HashMap::new();
    seen.entry(start_pos).or_insert(HashSet::new()).insert('^');
    let mut new_pos = move_guard(start_pos, field);
    while 0 <= new_pos.0
        && new_pos.0 < field.len() as i32
        && 0 <= new_pos.1
        && new_pos.1 < field[0].len() as i32
    {
        let dir_pos = new_pos;
        let dir_char: char = field[new_pos.0 as usize][new_pos.1 as usize];
        if seen.contains_key(&dir_pos)
            && seen
                .get(&dir_pos)
                .expect("Should be there")
                .contains(&dir_char)
        {
            return true;
        }
        seen.entry(dir_pos)
            .or_insert(HashSet::new())
            .insert(dir_char);
        new_pos = move_guard(new_pos, field);
    }

    false
}

fn check_blocks(
    start_pos: (i32, i32),
    field: &mut Vec<Vec<char>>,
    checks: &HashSet<(i32, i32)>,
) -> i32 {
    field[start_pos.0 as usize][start_pos.1 as usize] = '^';
    let mut sum = 0;
    for pos in checks {
        let i = pos.0 as usize;
        let j = pos.1 as usize;
        field[i][j] = '#';
        if move_guard_until_loop_or_off_map(start_pos, field) {
            sum += 1;
        }
        field[i][j] = '.';
        field[start_pos.0 as usize][start_pos.1 as usize] = '^';
    }
    sum
}

pub fn solve() {
    let (mut test_data, test_start_pos) = load_data("src/day6/test.txt");
    let (mut data, start_pos) = load_data("src/day6/input.txt");

    let (test_unique_spaces, test_seen) = move_guard_until_off_map(test_start_pos, &mut test_data);
    let test_unique_blocks = check_blocks(test_start_pos, &mut test_data, &test_seen);
    let (unique_spaces, seen) = move_guard_until_off_map(start_pos, &mut data);
    let unique_blocks = check_blocks(start_pos, &mut data, &seen);

    println!("Test result: {}", test_unique_spaces);
    println!("Test blocks: {}", test_unique_blocks);
    println!("Result: {}", unique_spaces);
    println!("Result blocks: {}", unique_blocks);
}
