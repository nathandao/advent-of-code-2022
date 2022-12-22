use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    let mut h_pos: Vec<i32> = vec![0, 0];
    let mut t_pos: Vec<i32> = vec![0, 0];
    let mut covered_pos: Vec<String> = vec![];

    for line in contents.lines() {
        let m = line.split(" ").collect::<Vec<&str>>();
        let direction = m[0];
        let steps = m[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "L" => h_pos[0] -= 1,
                "R" => h_pos[0] += 1,
                "U" => h_pos[1] -= 1,
                "D" => h_pos[1] += 1,
                _ => {}
            }
            if !is_overlapping(&h_pos, &t_pos) && !is_touching(&h_pos, &t_pos) {
                if (h_pos[0] - t_pos[0]).abs() == 2 {
                    t_pos[0] += (h_pos[0] - t_pos[0]) / 2;

                    if (h_pos[1] - t_pos[1]).abs() == 1 {
                        t_pos[1] += h_pos[1] - t_pos[1];
                    }
                }

                if (h_pos[1] - t_pos[1]).abs() == 2 {
                    t_pos[1] += (h_pos[1] - t_pos[1]) / 2;

                    if (h_pos[0] - t_pos[0]).abs() == 1 {
                        t_pos[0] += h_pos[0] - t_pos[0];
                    }
                }
            }

            let covered = t_pos
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            if !covered_pos.contains(&covered) {
                covered_pos.push(covered);
            }
        }
    }

    println!("Part 1: {}", covered_pos.len());
}

fn is_overlapping(h: &Vec<i32>, t: &Vec<i32>) -> bool {
    return h[0] == t[0] && h[1] == t[1];
}

fn is_touching(h: &Vec<i32>, t: &Vec<i32>) -> bool {
    return (h[0] - t[0]).abs() <= 1 && (h[1] - t[1]).abs() <= 1 && !is_overlapping(h, t);
}
