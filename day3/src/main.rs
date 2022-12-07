use array_tool::vec::Intersect;
use std::fs;

#[derive(Debug)]
struct Rucksack {
    upper_compartment: Vec<char>,
    lower_compartment: Vec<char>,
}

impl Rucksack {
    fn misplace_score(&self, alphabet: &Vec<char>) -> i32 {
        let intersection = self
            .lower_compartment
            .intersect(self.upper_compartment.to_owned());
        print!("{:?}", intersection[0]);

        return alphabet.iter().position(|&r| r == intersection[0]).unwrap() as i32 + 1;
    }
}

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let input = fs::read_to_string("input.txt").expect("Data not found");
    let rucksacks = input
        .split("\n")
        .map(move |rucksack| {
            let items = rucksack.len();
            let parsed_rucksack = Rucksack {
                upper_compartment: rucksack[..(items / 2)].chars().collect(),
                lower_compartment: rucksack[(items / 2)..].chars().collect(),
            };
            return parsed_rucksack.misplace_score(&alphabet);
        })
        .collect::<Vec<i32>>();
    println!("{:?}", rucksacks.iter().sum::<i32>())
}
