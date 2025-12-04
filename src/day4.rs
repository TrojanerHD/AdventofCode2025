#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
pub fn part1(input: &str) -> String {
    let points = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '@' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    for point in points.iter() {
        let mut count = 0;
        let y_minus = point.y.checked_sub(1).unwrap_or(point.y);
        for y in y_minus..=point.y + 1 {
            let x_minus = point.x.checked_sub(1).unwrap_or(point.x);
            for x in x_minus..=point.x + 1 {
                if x == point.x && y == point.y {
                    continue;
                }

                if points.iter().any(|point2| point2.x == x && point2.y == y) {
                    count += 1;
                }
            }
        }
        if count < 4 {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut points = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '@' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    let mut new_points = points.clone();
    let mut first_loop = true;
    while first_loop || new_points != points {
        first_loop = false;
        points = new_points.clone();
        for point in points.iter() {
            let mut count = 0;
            let y_minus = point.y.checked_sub(1).unwrap_or(point.y);
            for y in y_minus..=point.y + 1 {
                let x_minus = point.x.checked_sub(1).unwrap_or(point.x);
                for x in x_minus..=point.x + 1 {
                    if x == point.x && y == point.y {
                        continue;
                    }

                    if points.iter().any(|point2| point2.x == x && point2.y == y) {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                res += 1;
                new_points.retain(|point2| point2.x != point.x || point2.y != point.y);
            }
        }
    }
    res.to_string().to_owned()
}
