use std::{collections::{HashMap, HashSet}, fs};

use regex::Regex;

fn load_data(path: &str) -> Vec<Vec<(i32, i32)>> {
    let data = fs::read_to_string(path).unwrap();
    let lines = data.lines();

    let mut data = Vec::new();
    let r = r"-?\d+";
    let re = Regex::new(r).unwrap();
    for line in lines {
        let mut row = Vec::new();
        let mut captures = re.captures_iter(line);
        row.push((
            captures.next().unwrap()[0].parse::<i32>().unwrap(),
            captures.next().unwrap()[0].parse::<i32>().unwrap(),
        ));
        row.push((
            captures.next().unwrap()[0].parse::<i32>().unwrap(),
            captures.next().unwrap()[0].parse::<i32>().unwrap(),
        ));
        data.push(row);
    }

    data
}

fn pos_after(start: (i32, i32), v: (i32, i32), n: i32, field_size: (i32, i32)) -> (i32, i32) {
    let mut p = (
        (n * v.0 + start.0) % field_size.0,
        (n * v.1 + start.1) % field_size.1,
    );

    if p.0 < 0 {
        p.0 += field_size.0;
    }

    if p.1 < 0 {
        p.1 += field_size.1;
    }

    p
}

fn in_quadrant(pos: (i32, i32), field_size: (i32, i32)) -> (i32, i32) {
    let q_l = field_size.0 / 2 - 1;
    let q_r = field_size.1 / 2 - 1;
    if pos.0 <= q_l && pos.1 <= q_r {
        return (0, 0);
    }
    if pos.0 <= q_l && pos.1 > q_r + 1 {
        return (0, 1);
    }
    if pos.0 > q_l + 1 && pos.1 <= q_r {
        return (1, 0);
    }
    if pos.0 > q_l + 1 && pos.1 > q_r + 1 {
        return (1, 1);
    }

    return (-1, -1);
}

fn print_robots(robots: &Vec<(i32, i32)>, field_size: (i32, i32)) {
    let mut field = Vec::new();
    for _ in 0..field_size.1 {
        let mut row = Vec::new();
        for _ in 0..field_size.0 {
            row.push('.');
        }
        field.push(row);
    }

    for robot in robots {
        field[robot.1 as usize][robot.0 as usize] = 'R';
    }

    for row in field {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn solve_per_path(path: &str, field_size: (i32, i32)) -> (i32, i32) {
    let data = load_data(path);

    let mut map = HashMap::new();
    map.insert((0, 0), 0);
    map.insert((0, 1), 0);
    map.insert((1, 0), 0);
    map.insert((1, 1), 0);

    for robot in &data {
        let p_100 = pos_after(robot[0], robot[1], 100, field_size);
        let quadrant = in_quadrant(p_100, field_size);
        if quadrant == (-1, -1) {
            continue;
        }
        *map.get_mut(&quadrant).unwrap() += 1;
    }

    let p1 = map.values().fold(1, |agg, v| agg * v);
    
    let mut not_found = true;
    let mut ct = 0;
    while not_found {
        ct += 1;

        let mut seen = HashSet::new();
        for robot in &data {
            let pos = pos_after(robot[0], robot[1], ct, field_size);
            if seen.contains(&pos) {
                break;
            }

            seen.insert(pos);
        }
        
        if seen.len() == data.len() {
            not_found = false;
            
            let mut robots = Vec::new();
            for robot in &data {
                robots.push(pos_after(robot[0], robot[1], ct, field_size)); 
            }

            print_robots(&robots, field_size);
        }
    }
    

    (p1, ct)
}

pub fn solve() {
    let test_field_size = (11, 7);
    let field_size = (101, 103);

    let (test_p1, test_p2) = solve_per_path(&"src/day14/test.txt", test_field_size);
    let (p1, p2) = solve_per_path(&"src/day14/input.txt", field_size);

    println!("Test result p1: {}", test_p1);
    println!("Result p1: {}", p1);
    println!("Result p2: {}", p2);
}
