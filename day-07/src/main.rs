use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    let mut current_dir = String::from("/");
    let mut directories: HashMap<String, usize> = HashMap::new();
    let re = Regex::new(r"^(\d+)").unwrap();
    let lines = contents.lines();

    for line in lines {
        match &line[..4] {
            "$ cd" => match &line[5..] {
                "/" => current_dir = String::from("/"),
                ".." => {
                    let paths = current_dir.split("/").collect::<Vec<&str>>();
                    current_dir = paths[..paths.len() - 2].join("/") + "/";
                }
                _ => current_dir = current_dir + &line[5..] + "/",
            },
            "$ ls" => {}
            "dir " => {}
            _ => {
                for cap in re.captures_iter(line) {
                    let file_size: usize = cap[1].parse().unwrap();
                    update_dir_size(&mut directories, current_dir.clone(), file_size);
                }
            }
        }
    }

    let part1_result = directories
        .values()
        .filter(|v| **v <= 100000)
        .sum::<usize>();

    println!("Part 1: {}", part1_result);
}

fn update_dir_size(directories: &mut HashMap<String, usize>, dir: String, file_size: usize) {
    directories
        .entry(dir.clone())
        .and_modify(|v| *v += file_size)
        .or_insert(file_size);

    let mut current_dir = dir.clone();

    while current_dir != "/" {
        let paths = current_dir.split("/").collect::<Vec<&str>>();
        current_dir = paths[..paths.len() - 2].join("/") + "/";
        directories
            .entry(current_dir.clone())
            .and_modify(|v| *v += file_size)
            .or_insert(file_size);
    }
}
