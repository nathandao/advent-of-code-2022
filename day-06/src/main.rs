use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let chars: Vec<char> = read_to_string(Path::new(&filename))
        .expect("unable to read from file")
        .chars()
        .collect();

    print!("Part 1: {:?}\n", find_marker(chars.clone(), 4));
    print!("Part 2: {:?}\n", find_marker(chars.clone(), 14));
}

fn find_marker(chars: Vec<char>, marker_len: usize) -> usize {
    let mut result: usize = 0;
    for i in 0..chars.len() - marker_len {
        let mut quad = chars[i..i + marker_len].to_vec();
        quad.sort();
        quad.dedup();

        if quad.len() == marker_len {
            result = i + marker_len;
            break;
        }
    }

    return result;
}
