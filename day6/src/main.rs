use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");
    let char_vec = input.chars().into_iter().collect::<Vec<char>>();

    for (i, char) in char_vec.iter().enumerate() {
        if (i >= 13) {
            let mut set: HashSet<char> = HashSet::new();
            for a in ((i - 13)..=i) {
                print!("{:?}", char_vec[a]);

                set.insert(char_vec[a]);
            }
            println!("{:?}", set);
            if (set.len() == 14) {
                println!("{:?}", set);
                println!("Found {}", i + 1);
                break;
            }
        }
    }
}
