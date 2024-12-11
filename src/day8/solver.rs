use std::{collections::HashSet, fs};

fn compute_antinode(node1: (i32, i32), node2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = node1.0 - node2.0;
    let dy = node1.1 - node2.1;

    ((node2.0 - dx, node2.1 - dy), (node1.0 + dx, node1.1 + dy))
}


fn compute_antinode_p2(node1: (i32, i32), node2: (i32, i32), field_size: (i32, i32)) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new(); 
    let dx = node1.0 - node2.0;
    let dy = node1.1 - node2.1;

    let mut v_d = (node2.0 - dx, node2.1 - dy);
    while in_field(v_d, field_size) {
        antinodes.push(v_d);

        v_d = (v_d.0 - dx, v_d.1 - dy);
    }
    
    v_d = (node1.0 + dx, node1.1 + dy);
    while in_field(v_d, field_size) {
        antinodes.push(v_d);

        v_d = (v_d.0 + dx, v_d.1 + dy);
    }

    antinodes
}


fn in_field(node: (i32, i32), field_size: (i32, i32)) -> bool {
    (0..field_size.0).contains(&node.0) && (0..field_size.1).contains(&node.1)
}

fn compute_antinodes(nodes: &Vec<(char, (i32, i32))>, field_size: (i32, i32), p2: bool) -> i32 {
    let mut seen = HashSet::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if nodes[i].0 == nodes[j].0 {
                if p2 {
                    let antinodes = compute_antinode_p2(nodes[i].1, nodes[j].1, field_size);
                    for antinode in antinodes {
                        seen.insert(antinode);
                    }
                    seen.insert(nodes[i].1);
                    seen.insert(nodes[j].1);

                } else {
                    let (antinode1, antinode2) = compute_antinode(nodes[i].1, nodes[j].1);
                    if in_field(antinode1, field_size) {
                        seen.insert(antinode1);
                    }
                    if in_field(antinode2, field_size) {
                        seen.insert(antinode2);
                    }
                }
            }
        }
    }

    seen.len() as i32
}

fn load_data(path: &str) -> (Vec<(char, (i32, i32))>, (i32, i32)) {
    let data = fs::read_to_string(path).expect("File should exist");
    let lines: Vec<Vec<char>> = data.lines().map(|c| c.chars().collect()).collect();

    let mut antennas = Vec::new();
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let ch = lines[i][j];
            if ch != '.' {
                antennas.push((ch, (i as i32, j as i32)));
            }
        }
    }

    (antennas, (lines.len() as i32, lines[0].len() as i32))
}

pub fn solve() {
    let test_path = "src/day8/test.txt";
    let (test_nodes, test_field_size) = load_data(test_path);
    let test_result = compute_antinodes(&test_nodes, test_field_size, false);
    let test_result_p2 = compute_antinodes(&test_nodes, test_field_size, true);
    
    let path = "src/day8/input.txt";
    let (nodes, field_size) = load_data(path);
    let result = compute_antinodes(&nodes, field_size, false);
    let result_p2 = compute_antinodes(&nodes, field_size, true);

    println!("Test result: {}", test_result);
    println!("Test result p2: {}", test_result_p2);
    println!("Result: {}", result);
    println!("Result p2: {}", result_p2);
}
