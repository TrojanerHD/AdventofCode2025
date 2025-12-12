#[derive(Debug, Clone)]
struct Request {
    x_size: u16,
    y_size: u16,
    shape_indices: Vec<usize>,
}

pub fn part1(input: &str) -> String {
    let mut shape_amount = 0;
    let mut requests = Vec::new();
    let mut done = false;
    for line in input.lines().skip(1) {
        if line.contains(":") {
            if line.contains("x") {
                if !done {
                    shape_amount += 1;
                    done = true;
                }
                let (size, indices) = line.split_once(": ").unwrap();
                let (x_size, y_size) = size.split_once("x").unwrap();
                requests.push(Request {
                    x_size: x_size.parse().unwrap(),
                    y_size: y_size.parse().unwrap(),
                    shape_indices: indices
                        .split_whitespace()
                        .map(|index| index.parse().unwrap())
                        .collect::<Vec<_>>(),
                });
                continue;
            }
            shape_amount += 1;
        }
    }

    requests
        .into_iter()
        .filter(|request| {
            request.x_size * request.y_size
                >= (0..shape_amount)
                    .map(|i| request.shape_indices[i] as u16 * 9)
                    .sum()
        })
        .count()
        .to_string()
        .to_owned()
}

pub fn part2(_input: &str) -> String {
    "".to_owned()
}
