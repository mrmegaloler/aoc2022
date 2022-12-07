use std::fs;

use std::collections::HashMap;

#[derive(Debug)]
struct Challenge {
    opponent: String,
    myself: String,
}

fn main() {
    let mut winners = HashMap::<String, i32>::new();
    let mut losers = HashMap::<String, i32>::new();
    let mut equals = HashMap::<String, i32>::new();

    let mut score: i32 = 0;

    winners.insert("C".to_string(), 1);
    winners.insert("B".to_string(), 3);
    winners.insert("A".to_string(), 2);

    losers.insert("C".to_string(), 2);
    losers.insert("B".to_string(), 1);
    losers.insert("A".to_string(), 3);

    equals.insert("C".to_string(), 3);
    equals.insert("B".to_string(), 2);
    equals.insert("A".to_string(), 1);

    let input = fs::read_to_string("input.txt").expect("Data not found");
    let challenge_row = input
        .split("\n")
        .map(|challenge| {
            let parsed_challenge = challenge.split_ascii_whitespace().collect::<Vec<&str>>();
            return Challenge {
                opponent: parsed_challenge[0].to_string(),
                myself: parsed_challenge[1].to_string(),
            };
        })
        .collect::<Vec<Challenge>>();

    for challenge in challenge_row.into_iter() {
        // score += option_score.get(&challenge.myself).unwrap();

        if challenge.myself == "Y" {
            score += 3 + equals.get(&challenge.opponent).unwrap();
        } else if challenge.myself == "Z" {
            score += 6 + winners.get(&challenge.opponent).unwrap();
        } else {
            score += losers.get(&challenge.opponent).unwrap()
        }
    }

    println!("{}", score)
}
