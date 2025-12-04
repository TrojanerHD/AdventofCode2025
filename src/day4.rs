fn check_surrounding(points: &[Vec<bool>], x: usize, y: usize) -> usize {
    let y_minus = y.checked_sub(1).unwrap_or(y);
    let x_minus = x.checked_sub(1).unwrap_or(x);
    points
        .iter()
        .enumerate()
        .take((y + 1).min(points.len() - 1) + 1)
        .skip(y_minus)
        .scan(0_usize, |acc, (y1, line)| -> Option<usize> {
            if *acc >= 4 {
                None
            } else {
                *acc += line
                    .iter()
                    .enumerate()
                    .take((x + 1).min(line.len() - 1) + 1)
                    .skip(x_minus)
                    .scan(0_usize, |acc2, (x1, &val)| {
                        if *acc2 >= 4 - *acc {
                            return None;
                        }
                        if (y1 != y || x1 != x) && val {
                            *acc2 += 1;
                        }
                        Some(*acc2)
                    })
                    .last()
                    .unwrap();
                Some(*acc)
            }
        })
        .last()
        .unwrap()
}

pub fn part1(input: &str) -> String {
    let points = input
        .lines()
        .map(|line| line.chars().map(|char| char == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (y, line) in points.iter().enumerate() {
        for (x, &val) in line.iter().enumerate() {
            if val && check_surrounding(&points, x, y) < 4 {
                res += 1;
            }
        }
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut points = input
        .lines()
        .map(|line| line.chars().map(|char| char == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    loop {
        let mut new_points = points.clone();
        let mut change = false;
        for (y, line) in points.iter().enumerate() {
            for (x, &val) in line.iter().enumerate() {
                if val && check_surrounding(&points, x, y) < 4 {
                    new_points[y][x] = false;
                    change = true;
                    res += 1;
                }
            }
        }
        if !change {
            break;
        }
        points = new_points;
    }

    res.to_string().to_owned()
}
