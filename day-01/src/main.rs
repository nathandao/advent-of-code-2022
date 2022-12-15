use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let contents = read_to_string(Path::new(&filename)).expect("Unable to read from file");

    let elfs = contents.split("\n\n").collect::<Vec<_>>();

    let mut total_calories = elfs
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .filter_map(|calories| calories.parse::<i32>().ok())
                .sum()
        })
        .collect::<Vec<i32>>();

    total_calories.sort_by(|a, b| b.cmp(a));

    print!(
        "Top 3 elfs: {}\n",
        total_calories[..3]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    print!(
        "Total top 3 elfs: {}\n",
        total_calories[..3].iter().sum::<i32>().to_string()
    )
}
