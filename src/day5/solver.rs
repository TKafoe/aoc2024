use std::fs;


fn shift_vector(v: &mut Vec<i32>, to_i: usize, from_i: usize) {
    let mut swap = v[from_i]; 
    for i in to_i..from_i + 1 {
        let t = v[i];
        v[i] = swap;

        swap = t;
    }
}


fn check_rule(update_list: &Vec<i32>, rule: &(i32, i32)) -> bool {
    let mut found = false;
    
    for update in update_list {
        if *update == rule.1 {
            found = true;
        }

        if found && *update == rule.0 {
            return false;
        }
    }

    true
}


fn fix_rule(update_list: &mut Vec<i32>, rule: &(i32, i32)) {
    let mut first_index = 0; 
    let mut found = false;
    for i in 0..update_list.len() {
        let update = update_list[i]; 
        if !found && update == rule.1 {
            first_index = i;
            found = true;
        }

        if found && update == rule.0 {
            shift_vector(update_list, first_index, i);
        }
    } 
}


fn check_all_rules(rules: &Vec<(i32, i32)>, update_list: &Vec<i32>) -> bool {
    for rule in rules {
        if !check_rule(&update_list, rule) {
            return false;
        }
    }
    true
}

fn sum_middle(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>, fix: bool) -> i32 {
    let mut sum = 0; 
    for update_list in updates {
        let mut mutable_update_list = update_list.clone();     
        let mut incorrect = false; 
        
        for rule in rules {
            if !check_rule(&mutable_update_list, rule) {
                incorrect = true;

                if fix {
                    fix_rule(&mut mutable_update_list, rule);
                } else {
                    break; 
                }
            }
        }
        
        while fix && !check_all_rules(rules, &mutable_update_list) {
            for rule in rules {
                if !check_rule(&mutable_update_list, rule) {
                    fix_rule(&mut mutable_update_list, rule);
                }
            }
        }
        
        if fix {
            assert!(check_all_rules(rules, &mutable_update_list));
        }

        if (!incorrect && !fix) || (incorrect && fix){
            sum += &mutable_update_list[&mutable_update_list.len() / 2];
        }
    }
    
    sum 
}



fn load_data(path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let file_str = fs::read_to_string(path).expect("Did not find file.");
    let lines = file_str.lines();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 4 {
            continue;
        }

        if chars[2] == '|' {
            let mut split = line.split("|");
            rules.push((
                split
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .expect("Input parsing went wrong."),
                split
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .expect("Input parsing went wrong."),
            ))
        } else if chars[2] == ',' {
            let split = line.split(',');
            let mut new_update = Vec::new();
            for update in split {
                new_update.push(update.parse::<i32>().expect("Input parsing went wrong."));
            }
            updates.push(new_update);
        }
    }

    (rules, updates)
}

pub fn solve() {
    let test_path = "src/day5/test.txt";
    let input_path = "src/day5/input.txt";
    let (test_rules, test_updates) = load_data(&test_path);
    let (rules, updates) = load_data(&input_path);

    let test_result = sum_middle(&test_rules, &test_updates, false);
    let result = sum_middle(&rules, &updates, false);
 
    let test_result_fix = sum_middle(&test_rules, &test_updates, true);
    let result_fix = sum_middle(&rules, &updates, true);
   
    println!("Test result: {}", test_result);
    println!("Input result: {}", result);

    println!("Test result fix: {}", test_result_fix);
    println!("Input result fix: {}", result_fix);
}
