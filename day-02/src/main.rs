use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    // PART 1

    // rock     rock
    // paper    paper
    // scissors scissors

    // D rock - rock: 0
    // W rock - paper: -1
    // L rock - scissors: -2

    // L paper - rock: 1
    // D paper - paper: 0
    // W paper - scissors: -1

    // W scissors - rock: 2
    // L scissors - paper: 1
    // D scissors - scissors: 0

    let mut points = HashMap::new();
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);

    let rounds = contents.split("\n").collect::<Vec<_>>();
    let mut total_points = 0;

    for round in rounds.clone() {
        let moves: Vec<&str> = round.split(" ").collect::<Vec<_>>();
        if moves.len() > 1 {
            let diff = *points.entry(moves[0]).or_default() - *points.entry(moves[1]).or_default();
            let p: i32;

            if diff == 0 {
                p = 3;
            } else if diff == -2 || diff == 1 {
                p = 0;
            } else {
                p = 6;
            }

            total_points += p + *points.entry(moves[1]).or_default();
        }
    }

    print!("Total points: {}\n", total_points.to_string());

    // PART 2

    // rock     rock
    // paper    paper
    // scissors scissors

    // D rock - rock: 0
    // W rock - paper: 1
    // L rock - scissors: 2

    // L paper - rock: 2
    // D paper - paper: 0
    // W paper - scissors: 1

    // W scissors - rock: 1
    // L scissors - paper: 2
    // D scissors - scissors: 0

    let moves_vec = ["A", "B", "C", "X", "Y", "Z"];

    let mut move_offset = HashMap::new();
    move_offset.insert("X", 2);
    move_offset.insert("Y", 0);
    move_offset.insert("Z", 1);

    total_points = 0;

    for round in rounds.clone() {
        let moves = round.split(" ").collect::<Vec<_>>();

        if moves.len() > 1 {
            let correct_move_idx = 3
                + (moves_vec.iter().position(|&r| r == moves[0]).unwrap()
                    + *move_offset.entry(moves[1]).or_default())
                    % 3;
            let diff = *points.entry(moves[0]).or_default()
                - *points.entry(moves_vec[correct_move_idx]).or_default();
            let p: i32;

            if diff == 0 {
                p = 3;
            } else if diff == -2 || diff == 1 {
                p = 0;
            } else {
                p = 6;
            }

            total_points += p + *points.entry(moves_vec[correct_move_idx]).or_default();
        }
    }

    print!("Second total: {}\n", total_points.to_string());
}
