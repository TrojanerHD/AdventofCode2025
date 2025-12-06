#[derive(PartialEq)]
enum Operation {
    Plus,
    Times,
}

pub fn part1(input: &str) -> String {
    let mut lines = vec![0_u64; input.lines().next().unwrap().split_whitespace().count()];
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
    let mut transpose_lines = vec![Vec::new(); input.lines().next().unwrap().len()];
    for line in transpose_lines.iter_mut() {
        *line = vec![' '; input.lines().count()];
    }

    for (y, line) in input.lines().rev().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '+' {
                operations.push(Operation::Plus);
                continue;
            } else if char == '*' {
                operations.push(Operation::Times);
                continue;
            }
            transpose_lines[x][y] = char;
        }
    }
    let act_lines = transpose_lines
        .iter()
        .map(|line| {
            line.iter().fold("".to_owned(), |acc, char| {
                if *char != ' ' {
                    char.to_string() + acc.as_str()
                } else {
                    acc
                }
            })
        })
        .collect::<Vec<_>>();

    let mut lines = vec![0_u64; input.lines().next().unwrap().split_whitespace().count()];
    let mut i = 0;
    for val in act_lines.iter() {
        if val.is_empty() {
            i += 1;
            continue;
        }
        let act_val = val.parse::<u64>().unwrap();
        if operations[i] == Operation::Plus {
            lines[i] += act_val;
        } else {
            if lines[i] == 0 {
                lines[i] = 1;
            }
            lines[i] *= act_val;
        }
    }

    lines.iter().sum::<u64>().to_string().to_owned()
}
