use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    let mut rope: Vec<Vec<i32>> = vec![vec![0, 0]; 2];
    let mut covered_pos: Vec<String> = vec![];

    for line in contents.lines() {
        make_move(&mut rope, &mut covered_pos, line);
    }

    println!("Part 1: {}", covered_pos.len());

    covered_pos = vec![];
    rope = vec![vec![0, 0]; 10];

    for line in contents.lines() {
        make_move(&mut rope, &mut covered_pos, line);
    }

    println!("Part 2: {}", covered_pos.len());
}

fn is_overlapping(h: &Vec<i32>, t: &Vec<i32>) -> bool {
    return h[0] == t[0] && h[1] == t[1];
}

fn is_touching(h: &Vec<i32>, t: &Vec<i32>) -> bool {
    return (h[0] - t[0]).abs() <= 1 && (h[1] - t[1]).abs() <= 1 && !is_overlapping(h, t);
}

fn make_move(rope: &mut Vec<Vec<i32>>, covered_pos: &mut Vec<String>, line: &str) {
    let m = line.split(" ").collect::<Vec<&str>>();
    let direction = m[0];
    let steps = m[1].parse::<i32>().unwrap();

    for _ in 0..steps {
        match direction {
            "L" => rope[0][0] -= 1,
            "R" => rope[0][0] += 1,
            "U" => rope[0][1] -= 1,
            "D" => rope[0][1] += 1,
            _ => {}
        }

        for i in 0..rope.len() - 1 {
            let h_pos = rope[i].clone();
            let t_pos = &mut rope[i + 1];

            if !is_overlapping(&h_pos, &t_pos) && !is_touching(&h_pos, &t_pos) {
                if (h_pos[0] - t_pos[0]).abs() == 2 {
                    t_pos[0] += (h_pos[0] - t_pos[0]) / 2;
                    if (h_pos[1] - t_pos[1]).abs() == 2 {
                        t_pos[1] += (h_pos[1] - t_pos[1]) / 2;
                    } else if (h_pos[1] - t_pos[1]).abs() == 1 {
                        t_pos[1] = h_pos[1];
                    }
                } else if (h_pos[1] - t_pos[1]).abs() == 2 {
                    t_pos[1] += (h_pos[1] - t_pos[1]) / 2;
                    if (h_pos[0] - t_pos[0]).abs() == 1 {
                        t_pos[0] = h_pos[0];
                    }
                }
            }
        }

        let covered = rope
            .last()
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        if !covered_pos.contains(&covered) {
            covered_pos.push(covered.clone());
        }
    }
}
