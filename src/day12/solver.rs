use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_data(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Should exist")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn in_field(pos: (i32, i32), field: &Vec<Vec<char>>) -> bool {
    pos.0 >= 0
        && pos.0 < field.len() as i32
        && pos.1 >= 0
        && pos.1 < field[pos.0 as usize].len() as i32
}

fn create_graph(input: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph = HashMap::new();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let val = input[i][j];
            let mut edges = Vec::new();
            if in_field(((i + 1) as i32, j as i32), input) && input[i + 1][j] == val {
                edges.push((i + 1, j));
            }
            if in_field((i as i32, (j + 1) as i32), input) && input[i][j + 1] == val {
                edges.push((i, j + 1));
            }
            if in_field(((i as i32 - 1), j as i32), input) && input[i - 1][j] == val {
                edges.push((i - 1, j));
            }
            if in_field((i as i32, (j as i32 - 1)), input) && input[i][j - 1] == val {
                edges.push((i, j - 1));
            }
            graph.insert((i, j), edges);
        }
    }

    graph
}

fn compute_corners(
    pos: (usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> usize {
    let edges = graph[&pos].clone();

    if edges.len() == 0 {
        return 4;
    }

    if edges.len() == 1 {
        return 2;
    }

    if edges.len() == 2 {
        let e1 = edges[0];
        let e2 = edges[1];
        if e1.0 == e2.0 || e1.1 == e2.1 {
            return 0;
        }
    
        let mut corners = 1;

        let e1_i = (e1.0 as i32, e1.1 as i32);
        let e2_i = (e2.0 as i32, e2.1 as i32);
        
        let a = (
            e1_i.0 - pos.0 as i32,
            e1_i.1 - pos.1 as i32,
        );
        let b = (
            e2_i.0 - pos.0 as i32,
            e2_i.1 - pos.1 as i32,
        );
        let check_pos = (pos.0 as i32 + a.0 + b.0, pos.1 as i32 + a.1 + b.1);
        if !graph[&edges[0]].contains(&(check_pos.0 as usize, check_pos.1 as usize)) {
            corners += 1;
        }
        
        return corners;
    }
    
    let mut corners = 0;
    for i in 0..edges.len() {
        let e1 = edges[i];
        let e2 = edges[(i + 1) % edges.len()];

        if e1.0 == e2.0 || e1.1 == e2.1 {
            continue;
        }

        let e1_i = (e1.0 as i32, e1.1 as i32);
        let e2_i = (e2.0 as i32, e2.1 as i32);
        
        let a = (
            e1_i.0 - pos.0 as i32,
            e1_i.1 - pos.1 as i32,
        );
        let b = (
            e2_i.0 - pos.0 as i32,
            e2_i.1 - pos.1 as i32,
        );
        let check_pos = (pos.0 as i32 + a.0 + b.0, pos.1 as i32 + a.1 + b.1);
        if !graph[&e2].contains(&(check_pos.0 as usize, check_pos.1 as usize)) {
            corners += 1;
        }
    }

    corners
}

fn compute_price_region(
    start: (usize, usize),
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> (usize, usize, HashSet<(usize, usize)>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);

    let mut per = 0;
    let mut area = 0;
    let mut corners = 0;
    while queue.len() != 0 {
        let cur = queue.pop().unwrap();
        if visited.contains(&cur) {
            continue;
        }

        visited.insert(cur);

        area += 1;
        let edges = graph[&cur].clone();
        if edges.len() != 4 {
            per += 4 - edges.len();
        }
        
        corners += compute_corners(cur, graph);

        for edge in edges {
            queue.push(edge);
        }
    }
    (per * area, corners * area, visited)
}

fn compute_all_perimeters(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>) -> (usize, usize) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut final_cost_p1 = 0;
    let mut final_cost_p2 = 0;
    for key in graph.keys() {
        if visited.contains(key) {
            continue;
        }

        let (cost_p1, cost_p2, group_visited) = compute_price_region(*key, graph);
        visited.extend(&group_visited);
        final_cost_p1 += cost_p1;
        final_cost_p2 += cost_p2;
    }

    (final_cost_p1, final_cost_p2)
}

pub fn solve() {
    let test_input_path = "src/day12/test.txt";
    let test_input_data = load_data(test_input_path);
    let test_input_graph = create_graph(&test_input_data);
    let (test_result_p1, test_result_p2) = compute_all_perimeters(&test_input_graph);
    println!("Test result_p1: {}", test_result_p1);
    println!("Test result_p2: {}", test_result_p2);

    let input_path = "src/day12/input.txt";
    let input_data = load_data(input_path);
    let input_graph = create_graph(&input_data);
    let (result_p1, result_p2) = compute_all_perimeters(&input_graph);
    println!("Result p1: {}", result_p1);
    println!("Result p2: {}", result_p2);
}
