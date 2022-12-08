use regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("stack.txt").expect("Data not found");
    let re = regex::Regex::new(r"(.{3}\s|.{3}\n|.{3}$)").expect("Failed");
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| re.find_iter(line).map(|m| m.as_str().trim()).collect())
        .collect();
    println!("{:?}", lines)
}
