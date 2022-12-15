use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("unable to read from file");
    let mut total_points = 0;

    let rucksacks: Vec<&str> = contents.lines().filter(|&l| l.len() > 0).collect();

    for rucksack in rucksacks.clone() {
        let item_count = rucksack.len();
        let items: Vec<char> = rucksack.chars().collect();
        let (first_comp, second_comp) = items.split_at(item_count / 2);

        for item in first_comp {
            if second_comp.contains(item) {
                total_points += get_priority(*item);
                break;
            }
        }
    }

    print!("Total points: {}\n", total_points);

    let groups: Vec<Vec<&str>> = rucksacks.clone().chunks(3).map(|s| s.into()).collect();

    total_points = 0;
    for group in groups.clone() {
        let group_rucksacks: Vec<Vec<char>> = group.iter().map(|l| l.chars().collect()).collect();
        let intersection: Vec<char> = group_rucksacks[0]
            .clone()
            .into_iter()
            .filter(|c| group_rucksacks[1].contains(c) && group_rucksacks[2].contains(c))
            .collect();

        total_points += get_priority(intersection[0]);
    }

    print!("Total points: {}\n", total_points);
}

fn get_priority(item: char) -> usize {
    let char_vec = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .collect::<Vec<char>>();

    return char_vec.iter().position(|&c| c == item).unwrap() + 1;
}
