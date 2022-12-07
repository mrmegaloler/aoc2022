use array_tool::vec::Intersect;
use std::{fs, ops::Index};

#[derive(Debug)]
struct Rucksack {
    upper_compartment: Vec<char>,
    lower_compartment: Vec<char>,
}

#[derive(Debug)]
struct GroupRucksacks {
    rucksack_one: Vec<char>,
    rucksack_two: Vec<char>,
    rucksack_three: Vec<char>,
}

impl Rucksack {
    fn misplace_score(&self, alphabet: &Vec<char>) -> i32 {
        let intersection = self
            .lower_compartment
            .intersect(self.upper_compartment.to_owned());

        return alphabet.iter().position(|&r| r == intersection[0]).unwrap() as i32 + 1;
    }
}

impl GroupRucksacks {
    fn misplace_score(&self, alphabet: &Vec<char>) -> i32 {
        let intersection = self
            .rucksack_one
            .intersect(self.rucksack_two.to_owned())
            .intersect(self.rucksack_three.to_owned());

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
        .map(|rucksack| {
            let items = rucksack.len();
            let parsed_rucksack = Rucksack {
                upper_compartment: rucksack[..(items / 2)].chars().collect(),
                lower_compartment: rucksack[(items / 2)..].chars().collect(),
            };
            return parsed_rucksack.misplace_score(&alphabet);
        })
        .collect::<Vec<i32>>();

    let all_rucksacks: Vec<GroupRucksacks> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let group = chunk.to_vec();
            return GroupRucksacks {
                rucksack_one: group[0].chars().collect(),
                rucksack_two: group[1].chars().collect(),
                rucksack_three: group[2].chars().collect(),
            };
        })
        .collect();

    let group_rucksack_score = all_rucksacks
        .iter()
        .map(|group_rucksack| {
            return group_rucksack.misplace_score(&alphabet);
        })
        .collect::<Vec<i32>>();
    println!("Score {}", group_rucksack_score.iter().sum::<i32>());
}
