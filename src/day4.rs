struct Point {
    x: usize,
    y: usize,
}

fn check_surrounding(points: &[Point], point: &Point) -> usize {
    let y_minus = point.y.checked_sub(1).unwrap_or(point.y);
    let x_minus = point.x.checked_sub(1).unwrap_or(point.x);

    points
        .iter()
        .filter(|point2| {
            point2.x >= x_minus
                && point2.x <= point.x + 1
                && point2.y >= y_minus
                && point2.y <= point.y + 1
                && (point2.x != point.x || point2.y != point.y)
        })
        .take(4)
        .count()
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
        if check_surrounding(&points, point) < 4 {
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
    let num_points = points.len();

    loop {
        let mut points_to_remove = points
            .iter()
            .enumerate()
            .filter_map(|(i, point)| {
                if check_surrounding(&points, point) < 4 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if points_to_remove.is_empty() {
            break;
        }
        points_to_remove.reverse();
        for i in points_to_remove {
            points.remove(i);
        }
    }

    (num_points - points.len()).to_string().to_owned()
}
