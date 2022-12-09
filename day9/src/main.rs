use std::{cmp, collections::HashSet, f64::MAX_EXP, fs};

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    fn move_head(&mut self, command: &str) -> (i32, i32) {
        let temp_head = (self.head.0.to_owned(), self.head.1.to_owned());
        match command {
            "U" => self.head.1 += 1,
            "D" => self.head.1 += -1,
            "L" => self.head.0 += -1,
            "R" => self.head.0 += 1,
            _ => println!("woops"),
        }

        if cmp::max(
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        ) > 1
        {
            self.tail.0 = temp_head.0;
            self.tail.1 = temp_head.1;
        }

        return (self.tail.0, self.tail.1);
    }
}

struct LongRope {
    segments: Vec<(i32, i32)>,
}

fn hypot(c1: (i32, i32), c2: (i32, i32)) -> ((i32, i32), f64) {
    return ((c2), ((c2.0 - c1.0) as f64).hypot((c2.1 - c1.1) as f64));
}

impl LongRope {
    fn move_head(&mut self, command: &str) -> (i32, i32) {
        match command {
            "U" => self.segments[0].1 += 1,
            "D" => self.segments[0].1 += -1,
            "L" => self.segments[0].0 += -1,
            "R" => self.segments[0].0 += 1,
            _ => println!("woops"),
        }

        for segment in 1..self.segments.len() {
            if cmp::max(
                (self.segments[segment - 1].0 - self.segments[segment].0).abs(),
                (self.segments[segment - 1].1 - self.segments[segment].1).abs(),
            ) > 1
            {
                let curr = self.segments[segment];
                let head = self.segments[segment - 1];
                //get closest coordinate
                let top = (head.0, (head.1 + 1));
                let bottom = (head.0, head.1 - 1);
                let left = (head.0 - 1, head.1);
                let right = (head.0 + 1, head.1);

                let min_dist = vec![
                    hypot(curr, top),
                    hypot(curr, bottom),
                    hypot(curr, right),
                    hypot(curr, left),
                ]
                .into_iter()
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

                println!(
                    "Head: {0:?} Curr: {1:?}, New: {2:?}",
                    head, curr, min_dist.0
                );

                self.segments[segment] = min_dist.0;
            }
        }

        return self.segments[9];
    }
}

fn part1(commands: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
    };

    let mut visited_coordinates: HashSet<(i32, i32)> = HashSet::new();

    for command in commands {
        let coordinate = rope.move_head(command);
        visited_coordinates.insert(coordinate);
    }

    return visited_coordinates;
}

fn part2(commands: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut rope = LongRope {
        segments: vec![(0, 0); 10],
    };

    let mut visited_coordinates: HashSet<(i32, i32)> = HashSet::new();

    for command in commands {
        let coordinate = rope.move_head(command);
        visited_coordinates.insert(coordinate);
    }

    return visited_coordinates;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");
    let commands: Vec<&str> = input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .flat_map(|command| {
            let number = command[1].parse::<i32>().unwrap();
            return std::iter::repeat(command[0])
                .take(number as usize)
                .collect::<Vec<&str>>();
        })
        .collect();

    println!("Part1 {:?}", part1(commands.to_owned()).len());
    println!("Part2 {:?}", part2(commands.to_owned()).len());
}
