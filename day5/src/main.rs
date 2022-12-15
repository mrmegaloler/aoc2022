use regex;
use std::fs;

#[derive(Clone, Debug)]
struct Move {
    amount: i32,
    from: i32,
    to: i32,
}

fn main() {
    let input = fs::read_to_string("stack.txt").expect("Data not found");
    let instruction_input = fs::read_to_string("instructions.txt").expect("Data not found");
    let re = regex::Regex::new(r"(.{3}\s|.{3}\n|.{3}$)").expect("Failed");
    let number_re = regex::Regex::new(r"\d+").expect("Failed");
    let instructions = instruction_input
        .lines()
        .map(|instruction| {
            let row_instruction: Vec<i32> = number_re
                .find_iter(instruction)
                .map(|m| {
                    return m.as_str().trim().to_owned().parse::<i32>().unwrap();
                })
                .collect();

            return Move {
                from: row_instruction[1],
                to: row_instruction[2],
                amount: row_instruction[0],
            };
        })
        .collect::<Vec<Move>>();

    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| re.find_iter(line).map(|m| m.as_str().trim()).collect())
        .collect();

    let mut columns: Vec<Vec<&str>> = vec![vec![]; 9];

    for row in lines.iter().rev() {
        for (index, col) in row.iter().enumerate() {
            if col.to_owned() != "" {
                columns[index].push(col);
            }
        }
    }

    for instruction in instructions {
        let mut temp_stack: Vec<&str> = vec![];
        for index in 1..=instruction.amount {
            let moving_box = columns[(instruction.from - 1) as usize].pop().unwrap();
            temp_stack.push(moving_box);
        }
        for index in 0..instruction.amount {
            columns[(instruction.to - 1) as usize].push(temp_stack.pop().unwrap());
        }
    }

    for index in columns {
        println!("{:?}", index.to_owned().pop().unwrap());
    }
    //println!("{:?}", columns);
}
