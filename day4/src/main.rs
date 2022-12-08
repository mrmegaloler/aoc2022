use std::fs;

#[derive(Debug)]
struct WorkPair {
    elf_one: (i32, i32),
    elf_two: (i32, i32),
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");
    let workorders: Vec<WorkPair> = input
        .split("\n")
        .map(|row| row.split(",").collect::<Vec<&str>>())
        .map(|elf| {
            let elf_one = elf[0]
                .split("-")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let elf_two = elf[1]
                .split("-")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            return WorkPair {
                elf_one: (elf_one[0], elf_one[1]),
                elf_two: (elf_two[0], elf_two[1]),
            };
        })
        .collect();

    let temp = workorders
        .iter()
        .map(|pair| {
            if (pair.elf_one.0 >= pair.elf_two.0 && pair.elf_one.1 <= pair.elf_two.1)
                || (pair.elf_two.0 >= pair.elf_one.0 && pair.elf_two.1 <= pair.elf_one.1)
            {
                return 1;
            } else {
                return 0;
            }
        })
        .collect::<Vec<i32>>();

    println!("{:?}", temp.iter().sum::<i32>())
}
