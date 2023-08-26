use std::collections::VecDeque;
use std::{fs, process};

pub fn day_5() {
    if let Ok(string_contents) = fs::read_to_string("./src/inputs/day5.txt") {
        println!(
            "Day 5 Solution: {}",
            process_input_and_execute_instructions(string_contents)
        );
    } else {
        eprintln!("Couldn't read the input file");
        process::exit(1);
    }
}

fn parse_data(line: &str, array_of_stacks: &mut Vec<VecDeque<char>>) {
    // for the first useful char the index of that char is 1, thereafter +4 to existing index
    let mut char_index = 1;
    let mut stack_index = 0;
    let chars: Vec<char> = line.chars().collect();
    while char_index < chars.len() && stack_index < array_of_stacks.len() {
        let concerned_char = chars[char_index];
        if concerned_char.is_alphabetic() {
            array_of_stacks[stack_index].push_back(concerned_char);
        }
        char_index += 4;
        stack_index += 1;
    }
}

fn execute_instructions(line: &str, array_of_stacks: &mut Vec<VecDeque<char>>) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let mut instruction_arr = vec![];
    for part in parts {
        if let Ok(num) = part.parse::<usize>() {
            instruction_arr.push(num)
        }
    }
    if instruction_arr.len() < 3 {
        eprintln!("Invalid Input Format!");
        process::exit(1);
    }
    let qty = instruction_arr[0];
    let from = instruction_arr[1] - 1;
    let to = instruction_arr[2] - 1;
    let mut transferr_arr: Vec<char> = Vec::new();
    for _ in 0..qty {
        if let Some(transfer_value) = array_of_stacks[from].pop_front() {
            transferr_arr.push(transfer_value);
            // part 1 soln
            // array_of_stacks[to].push_front(transfer_value);
        }
    }
    // part 2 soln
    for ele in transferr_arr.iter().rev().cloned() {
        array_of_stacks[to].push_front(ele);
    }
}

fn init_data_structure(total_stacks: usize, array_of_stacks: &mut Vec<VecDeque<char>>) {
    for _i in 0..total_stacks {
        array_of_stacks.push(VecDeque::new());
    }
}

fn process_input_and_execute_instructions(input: String) -> String {
    let mut instructions_phase = false;
    let mut array_of_stacks: Vec<VecDeque<char>> = vec![];
    let mut soln_string: String = String::new();
    for (index, line) in input.lines().enumerate() {
        // initialize the data structure
        if index == 0 {
            let total_stacks: usize = (line.chars().count() + 1) / 4;
            init_data_structure(total_stacks, &mut array_of_stacks);
        }
        if line.len() == 0 {
            instructions_phase = true;
            continue;
        }
        if !instructions_phase {
            parse_data(line, &mut array_of_stacks);
        } else {
            execute_instructions(line, &mut array_of_stacks);
        }
    }
    for stack in array_of_stacks {
        if stack.len() > 0 {
            let ele = stack[0];
            if ele.is_alphabetic() {
                soln_string = soln_string + &ele.to_string();
            }
        }
    }
    soln_string
}
