use std::fs;

fn load_data(path: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(path)
        .expect("Should exist")
        .lines()
        .map(|c| {
            c.chars()
                .map(|d| d.to_digit(10).expect("Should be a digit") as usize)
                .collect()
        })
        .collect()
}

fn in_field(pos: (i32, i32), field: &Vec<Vec<usize>>) -> bool {
    if pos.0 >= field.len() as i32
        || pos.0 < 0
        || pos.1 >= field[pos.0 as usize].len() as i32
        || pos.1 < 0
    {
        return false;
    }
    true
}

fn dfs_trailhead(
    pos: (i32, i32),
    input: &Vec<Vec<usize>>,
    use_unique: bool,
    found: &mut Vec<(i32, i32)>,
) -> i32 {
    let cur_val = input[pos.0 as usize][pos.1 as usize] as i32;
    if cur_val == 9 {
        if use_unique && !found.contains(&pos) {
            found.push(pos);
            return 1;
        }
        if !use_unique {
            return 1;
        }
        return 0;
    }

    let mut sum = 0;
    if in_field((pos.0 + 1, pos.1), input)
        && input[(pos.0 + 1) as usize][pos.1 as usize] as i32 == cur_val + 1
    {
        sum += dfs_trailhead((pos.0 + 1, pos.1), input, use_unique, found);
    }
    if in_field((pos.0 - 1, pos.1), input)
        && input[(pos.0 - 1) as usize][pos.1 as usize] as i32 == cur_val + 1
    {
        sum += dfs_trailhead((pos.0 - 1, pos.1), input, use_unique, found);
    }

    if in_field((pos.0, pos.1 - 1), input)
        && input[pos.0 as usize][(pos.1 - 1) as usize] as i32 == cur_val + 1
    {
        sum += dfs_trailhead((pos.0, pos.1 - 1), input, use_unique, found);
    }

    if in_field((pos.0, pos.1 + 1), input)
        && input[pos.0 as usize][(pos.1 + 1) as usize] as i32 == cur_val + 1
    {
        sum += dfs_trailhead((pos.0, pos.1 + 1), input, use_unique, found);
    }

    sum
}

fn solve_p1_p2(path: &str) -> (i32, i32) {
    let data = load_data(path);

    let mut starts = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 0 {
                starts.push((i as i32, j as i32));
            }
        }
    }

    (
        starts.iter().fold(0, |acc, &s| {
            acc + dfs_trailhead(s, &data, true, &mut Vec::new())
        }),
        starts.iter().fold(0, |acc, &s| {
            acc + dfs_trailhead(s, &data, false, &mut Vec::new())
        }),
    )
}

pub fn solve() {
    let test_path = "src/day10/test.txt";
    let (test_result_p1, test_result_p2) = solve_p1_p2(test_path);
    println!("Test result p1: {}", test_result_p1);
    println!("Test result p2: {}", test_result_p2);

    let path = "src/day10/input.txt";
    let (result_p1, result_p2) = solve_p1_p2(path);
    println!("Result p1: {}", result_p1);
    println!("Result p2: {}", result_p2);
}
