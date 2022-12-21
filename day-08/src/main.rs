use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");
    let grid = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let cols = grid[0].len();
    let rows = grid.len();
    let mut visible_count = cols * 2 + (rows - 2) * 2;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let h = &grid[i][j];
            if h > &grid[..i]
                .iter()
                .map(|v| v[j])
                .collect::<Vec<usize>>()
                .iter()
                .max()
                .unwrap()
                || h > &grid[i + 1..]
                    .iter()
                    .map(|v| v[j])
                    .collect::<Vec<usize>>()
                    .iter()
                    .max()
                    .unwrap()
                || h > &grid[i][..j].iter().max().unwrap()
                || h > &grid[i][j + 1..].iter().max().unwrap()
            {
                visible_count += 1;
            }
        }
    }

    println!("Part 1: {}", visible_count);
}
