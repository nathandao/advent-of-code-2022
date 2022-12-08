use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("Unable to read from file");

    let elfs = contents.split("\n\n").collect::<Vec<_>>();

    let total_calories = elfs
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .filter_map(|calories| calories.parse::<usize>().ok())
                .sum()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut max = 0;
    for (idx, total) in total_calories.iter().enumerate() {
        if *total > max {
            max = *total;
            result = idx + 1;
        }
    }

    print!("Result: {}\n", result)
}
