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

    let mut max_score: usize = 0;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let tree_h = grid[i][j];
            let mut left: usize = 1;
            let mut right: usize = 1;
            let mut top: usize = 1;
            let mut down: usize = 1;

            // left
            let mut current_h = 0;
            while current_h < tree_h && left < j {
                current_h = grid[i][j - left - 1];
                if current_h <= tree_h {
                    left += 1;
                }
            }

            // right
            current_h = 0;
            while current_h < tree_h && right < cols - j - 1 {
                current_h = grid[i][j + right + 1];
                if current_h <= tree_h {
                    right += 1;
                }
            }

            // top
            current_h = 0;
            while current_h < tree_h && top < i {
                current_h = grid[i - top - 1][j];
                if current_h <= tree_h {
                    top += 1;
                }
            }

            // down
            current_h = 0;
            while current_h < tree_h && down < rows - i - 1 {
                current_h = grid[i + down + 1][j];
                if current_h <= tree_h {
                    down += 1;
                }
            }

            let score = left * right * top * down;
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 2: {}", max_score);
}
