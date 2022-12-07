use std::fs;

struct WorkPairs {
    elf_one: Vec<i32>,
    elf_two: Vec<i32>,
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");
    let workorders: Vec<Vec<Vec<&str>>> = input
        .split("\n")
        .map(|row| row.split(",").collect::<Vec<&str>>())
        .map(|elf| {
            let elf_one = elf[0].split("-").collect::<Vec<&str>>();
            let elf_two = elf[1].split("-").collect::<Vec<&str>>();
            return vec![elf_one, elf_two];
        })
        .collect();
    println!("{:?}", workorders)
}
