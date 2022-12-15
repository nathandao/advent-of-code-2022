use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    let fully_contained_pairs: Vec<Vec<&str>> = contents
        .lines()
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .filter(|p| {
            if p.len() != 2 {
                return false;
            }

            let first: Vec<usize> = p[0]
                .split("-")
                .map(|s| s.parse::<usize>().expect("unable to parse number"))
                .collect();

            let second: Vec<usize> = p[1]
                .split("-")
                .map(|s| s.parse::<usize>().expect("unable to parse number"))
                .collect();

            first[0] == second[0]
                || (first[0] < second[0] && first[1] >= second[1])
                || (first[0] > second[0] && first[1] <= second[1])
        })
        .collect();

    print!("Part 1 result: {}\n", fully_contained_pairs.len());

    let pairs: Vec<Vec<&str>> = contents
        .lines()
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .filter(|p| {
            if p.len() != 2 {
                return false;
            }

            let first: Vec<usize> = p[0]
                .split("-")
                .map(|s| s.parse::<usize>().expect("unable to parse number"))
                .collect();

            let second: Vec<usize> = p[1]
                .split("-")
                .map(|s| s.parse::<usize>().expect("unable to parse number"))
                .collect();

            first[0] == second[0]
                || (first[0] < second[0] && first[1] >= second[0])
                || (first[0] > second[0] && first[0] <= second[1])
        })
        .collect();

    print!("Part 2 result: {}\n", pairs.len());
}
