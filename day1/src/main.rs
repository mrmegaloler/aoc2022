use itertools::Itertools;
use std::fs;

fn main() {
    let calorie_rows = fs::read_to_string("input.txt").expect("Data not found");
    let calories_per_elf: Vec<i32> = calorie_rows
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|entry| entry.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    let sorted_calories_per_elf: Vec<i32> = calories_per_elf.into_iter().sorted().collect_vec();
    let rev: Vec<i32> = sorted_calories_per_elf.into_iter().rev().collect();
    println!("{0}, {1}, {2}", rev[0], rev[1], rev[2]);
    println!("{}", rev[0] + rev[1] + rev[2])
}
