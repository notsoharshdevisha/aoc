use std::fs;

#[derive(Debug)]
enum LogType {
    Command(Vec<String>),
    Output(Vec<String>),
}

struct Folder {
    files: Vec<String>,
    folders: Box<Folder>,
}

struct FileSystem {
    root: Folder,
}

pub fn day_7() {
    if let Ok(logs) = fs::read_to_string("./src/inputs/day7.txt") {
        parse_commands(&logs);
    } else {
        eprintln!("Couldn't read the input file")
    }
}

fn parse_commands(logs: &str) {
    for log in logs.lines().map(|line| line.trim()) {
        let log_parts: Vec<&str> = log.split_whitespace().collect();
        if let Ok(log_type) = identify_log_type(log_parts) {
            match log_type {
                LogType::Command(log_parts) => process_command(log_parts),
                LogType::Output(log_parts) => process_output(log_parts),
            }
        } else {
            eprintln!("Invalid Input Format")
        }
    }
}

fn identify_log_type(log_parts: Vec<&str>) -> Result<LogType, &str> {
    if log_parts[0] == "$" {
        return Ok(LogType::Command(get_owned_from_ref(log_parts)));
    } else if log_parts[0].len() > 0 {
        return Ok(LogType::Output(get_owned_from_ref(log_parts)));
    } else {
        return Result::Err("Invalid Input Format");
    }
}

fn get_owned_from_ref(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|part| (*part).to_owned()).collect()
}

fn process_command(log_parts: Vec<String>) {}

fn process_output(log_parts: Vec<String>) {}
