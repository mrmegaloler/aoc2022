use std::fs;
#[derive(Debug)]

struct Node {
    name: String,
    size: i32,
    children: Vec<Node>,
}
#[derive(Debug)]
struct Command {
    command: String,
    output: Vec<String>,
}

impl Node {
    fn handle_commands(
        mut self,
        mut commands: &mut Vec<Command>,
        mut size_counter: &mut i64,
    ) -> Node {
        while commands.len() > 0 {
            let command = commands.pop().unwrap();
            if command.command == "cd .." {
                self.size = self.children.iter().map(|x| x.size).sum();
                if self.size <= 100000 && self.children.len() > 0 {
                    *size_counter += self.size as i64;
                }
                return self;
            }

            if command.command == "ls" {
                for output in command.output.iter() {
                    if !output.starts_with("dir") {
                        let it = output.split_ascii_whitespace().collect::<Vec<&str>>();
                        self.children.push(Node {
                            children: vec![],
                            size: it[0].parse::<i32>().unwrap(),
                            name: it[1].to_owned(),
                        })
                    }
                }
            }

            if command.command.starts_with("cd") {
                let it = command
                    .command
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>();
                let mut new_node = Node {
                    children: vec![],
                    size: -1,
                    name: it[1].to_owned(),
                };
                let node = new_node.handle_commands(commands, size_counter);
                self.children.push(node);
            }
        }

        return self;
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Data not found");
    let command_with_output = input
        .split("$ ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|it| {
            it.split("\n")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let mut commands = command_with_output
        .iter()
        .rev()
        .map(|it| Command {
            command: it[0].to_owned(),
            output: it
                .iter()
                .skip(1) // Skip the first element
                .cloned() // Clone each element
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
        })
        .collect::<Vec<Command>>();

    let mut node = Node {
        children: vec![],
        name: "/".to_string(),
        size: -1,
    };

    let mut size = 0;
    let result = node.handle_commands(&mut commands, &mut size);

    println!("{:?}", size)

    // let commands: Vec<Command> =

    // let tree: Node = Node {
    //     size: -1,
    //     children: vec![],
    // };
}
