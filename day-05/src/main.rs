use regex::Regex;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use std::vec;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");

    let part1 = solve(contents.clone(), true);
    let part2 = solve(contents.clone(), false);

    print!("Part 1: {}\nPart 2: {}\n", part1, part2);
}

fn solve(contents: String, use_rev: bool) -> String {
    let content_lines: Vec<&str> = contents.lines().collect();
    let stack_lines: Vec<&str> = content_lines[0..8].to_vec();
    let mut stacks: Vec<Vec<char>> = vec::from_elem(vec![], 9);

    for stack_line in stack_lines.into_iter().rev().collect::<Vec<&str>>() {
        let stack_chars: Vec<char> = stack_line
            .chars()
            .enumerate()
            .filter(|&(idx, _)| idx > 0 && (idx - 1) % 4 == 0)
            .map(|(_, e)| e)
            .collect();
        for i in 0..(stack_chars.len()) {
            if stack_chars[i] != ' ' {
                stacks[i].push(stack_chars[i]);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    let mut moves: Vec<Vec<usize>> = vec![];
    for cap in re.captures_iter(&contents) {
        moves.push(vec![
            cap[1].parse::<usize>().expect("unable to parse number"),
            cap[2].parse::<usize>().expect("unable to parse number") - 1,
            cap[3].parse::<usize>().expect("unable to parse number") - 1,
        ]);
    }

    for m in moves.clone() {
        let count = m[0];
        let source = &mut stacks[m[1]];
        let mut stash = source[(source.len() - count)..].to_vec();

        source.truncate(source.len() - stash.len());

        if use_rev {
            let mut_stash = &mut stash.iter().rev().map(|c| *c).collect::<Vec<char>>();
            stacks[m[2]].append(mut_stash);
        } else {
            let mut_stash = &mut stash;
            stacks[m[2]].append(mut_stash);
        }
    }

    return stacks
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");
}
