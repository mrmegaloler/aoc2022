use std::fs;
use std::ptr;

#[derive(Debug)]

struct Tree {
    value: i32,
    visible: bool,
    scenic_score: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");

    let trees: Vec<i32> = input
        .chars()
        .filter(|val| val.is_digit(10))
        .map(|val| val.to_digit(10).unwrap() as i32)
        .collect();
    let mut tree_grid: Vec<Vec<Tree>> = trees
        .chunks(99)
        .map(|chunk| {
            chunk
                .iter()
                .map(|tree| Tree {
                    value: tree.to_owned(),
                    visible: false,
                    scenic_score: 0,
                })
                .collect::<Vec<Tree>>()
        })
        .collect();

    // let to

    for x in 0..99 {
        let mut column_max = -1;
        let mut row_max = -1;
        for y in 0..99 {
            if tree_grid[x][y].value > column_max {
                column_max = tree_grid[x][y].value;
                tree_grid[x][y].visible = true
            }
            if tree_grid[y][x].value > row_max {
                row_max = tree_grid[y][x].value;
                tree_grid[y][x].visible = true
            }
        }
    }

    for x in (0..99).rev() {
        let mut column_max = -1;
        let mut row_max = -1;
        for y in (0..99).rev() {
            if tree_grid[x][y].value > column_max {
                column_max = tree_grid[x][y].value;
                tree_grid[x][y].visible = true
            }
            if tree_grid[y][x].value > row_max {
                row_max = tree_grid[y][x].value;
                tree_grid[y][x].visible = true
            }
        }
    }
    let mut visible_trees = 0;

    for x in 0..99 {
        for y in 0..99 {
            if tree_grid[x][y].visible {
                visible_trees += 1;
            }
        }
    }

    let mut max_score = 0;

    for x in 0..99 {
        for y in 0..99 {
            let mut top: i32 = 0;
            let mut bottom: i32 = 0;
            let mut left: i32 = 0;
            let mut right: i32 = 0;

            for top_distance in 1..=y {
                if tree_grid[y][x].value <= tree_grid[y - top_distance][x].value {
                    top = top_distance as i32;
                    break;
                }
                top = top_distance as i32;
            }

            for left_distance in 1..=x {
                if tree_grid[y][x].value <= tree_grid[y][x - left_distance].value {
                    left = left_distance as i32;
                    break;
                }
                left = left_distance as i32;
            }

            for bottom_distance in (y + 1)..99 {
                if tree_grid[y][x].value <= tree_grid[bottom_distance][x].value {
                    bottom = (bottom_distance - y) as i32;
                    break;
                }
                bottom = (bottom_distance - y) as i32;
            }

            for right_distance in (x + 1)..99 {
                if tree_grid[y][x].value <= tree_grid[y][right_distance].value {
                    right = (right_distance - x) as i32;
                    break;
                }
                right = (right_distance - x) as i32;
            }

            let scenic_score = top * bottom * left * right;

            if (x > 0 && y > 0) {
                println!("{}", scenic_score);
            }
            if max_score < scenic_score {
                max_score = scenic_score
            }
        }
    }

    println!("{:?}", max_score);
}
