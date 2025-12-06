#[derive(PartialEq)]
enum Operation {
    Plus,
    Times,
}

pub fn part1(input: &str) -> String {
    let mut lines = vec![0_u64; input.lines().next().unwrap().split_whitespace().count()];
    // println!("{:?}", lines);
    let mut operations = Vec::new();
    let mut first = true;
    for line in input.lines().rev() {
        if first {
            for symbol in line.split_whitespace() {
                if symbol == "+" {
                    operations.push(Operation::Plus);
                } else {
                    operations.push(Operation::Times);
                }
            }
            first = false;
        } else {
            for (i, val) in line.split_whitespace().enumerate() {
                // println!("{val}");
                if operations[i] == Operation::Plus {
                    lines[i] += val.parse::<u64>().unwrap();
                } else {
                    if lines[i] == 0 {
                        lines[i] = 1;
                    }
                    lines[i] *= val.parse::<u64>().unwrap();
                }
            }
        }
    }

    lines.iter().sum::<u64>().to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut operations: Vec<Operation> = Vec::new();
    let mut act_lines = vec![Vec::new(); input.lines().next().unwrap().len()];
    for line in act_lines.iter_mut() {
        *line = vec!['a'; input.lines().count()];
    }

    for (y, line) in input.lines().rev().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '+' {
                operations.push(Operation::Plus);
                act_lines[x][y] = ' ';
                continue;
            } else if char == '*' {
                operations.push(Operation::Times);
                act_lines[x][y] = ' ';
                continue;
            }
            act_lines[x][y] = char;
        }
    }
    let act_lines = act_lines
        .iter()
        .map(|line| {
            line.iter()
                .fold("".to_owned(), |acc, char| char.to_string() + acc.as_str())
        })
        .collect::<Vec<_>>();
    // println!("{:?}", act_lines);
    // println!("{:?}", operations);
    // println!("{}", operations.len());
    // println!("{}", act_lines.len());
    let mut lines = vec![0_u64; input.lines().next().unwrap().split_whitespace().count()];
    let mut i = 0;
    for line in act_lines.iter() {
        // println!("Line: {line}");
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        for val in line.split_whitespace() {
            let act_val = val.parse::<u64>().unwrap();
            // println!("{act_val}, {act_i}");
            if operations[i] == Operation::Plus {
                // if i <= 6 {
                // println!("{} + {act_val} for {i}", lines[i]);
                // }
                lines[i] += act_val;
            } else {
                if lines[i] == 0 {
                    lines[i] = 1;
                }
                // if i <= 6 {
                // println!("{} * {act_val} for {i}", lines[i]);
                // }
                lines[i] *= act_val;
            }
        }
    }

    lines.iter().sum::<u64>().to_string().to_owned()
}
