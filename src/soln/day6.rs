use std::collections::HashSet;
use std::fs;

pub fn day_6() {
    if let Ok(data_stream) = fs::read_to_string("./src/inputs/day6.txt") {
        println!("day6 part1: {}", get_marker_index(&data_stream, 4 as usize));
        println!(
            "day6 part2: {}",
            get_marker_index(&data_stream, 14 as usize)
        )
    } else {
        eprintln!("error while read the input file");
    }
}

fn is_marker_valid(marker: &str, min_unique_char: usize) -> bool {
    let character_set: HashSet<char> = marker.chars().collect();
    if character_set.len() < min_unique_char {
        return false;
    }
    true
}

fn get_marker_index(data_stream: &str, min_unique_char: usize) -> usize {
    let mut marker_start_index = 0;
    let mut marker_end_index = min_unique_char;
    while marker_end_index <= data_stream.len() {
        if is_marker_valid(
            &data_stream[marker_start_index..marker_end_index],
            min_unique_char,
        ) {
            return marker_end_index;
        }
        marker_start_index += 1;
        marker_end_index += 1;
    }
    0
}
