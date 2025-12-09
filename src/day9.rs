#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn abs_diff(&self, other: &Point) -> Point {
        Self {
            x: self.x.abs_diff(other.x),
            y: self.y.abs_diff(other.y),
        }
    }
}

fn wrapping(index: usize, sub: bool, len: usize) -> usize {
    if !sub {
        if index + 1 < len - 1 { index + 1 } else { 0 }
    } else if let Some(sub) = index.checked_sub(1) {
        sub
    } else {
        len - 1
    }
}

pub fn part1(input: &str) -> String {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut max = 0;
    let mut other_points = Vec::new();
    for point1 in points {
        for point2 in other_points.iter() {
            let sub = point1.abs_diff(point2);
            let area = (sub.x + 1) * (sub.y + 1);
            max = max.max(area);
        }
        other_points.push(point1);
    }

    max.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut y_max = 0;
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            y_max = y_max.max(y);
            Point { x, y }
        })
        .collect::<Vec<_>>();

    let mut min_x: Option<(usize, Point)> = None;
    let mut max_x: Option<(usize, Point)> = None;
    let mut green = vec![(0, 0); y_max + 1];
    let mut min = None;
    let mut max = None;
    for (y, val) in green.iter_mut().enumerate() {
        let mut y_points = points
            .iter()
            .enumerate()
            .filter(|(_, point)| point.y == y)
            .map(|(i, point)| (i, *point))
            .collect::<Vec<_>>();
        y_points.sort_unstable_by_key(|(_, point)| point.x);
        if y_points.is_empty() {
            if let Some(min_x) = min_x
                && let Some(max_x) = max_x
            {
                let mut before_min = wrapping(min_x.0, true, points.len());
                let mut after_min = wrapping(min_x.0, false, points.len());
                let mut before_max = wrapping(max_x.0, true, points.len());
                let mut after_max = wrapping(max_x.0, false, points.len());

                while points[before_min].y <= y && points[after_min].y <= y {
                    before_min = wrapping(before_min, true, points.len());
                    after_min = wrapping(after_min, false, points.len());
                }

                while points[before_max].y <= y && points[after_max].y <= y {
                    before_max = wrapping(before_max, true, points.len());
                    after_max = wrapping(after_max, false, points.len());
                }

                if points[before_min].y > y {
                    min = Some(points[before_min].x);
                } else {
                    min = Some(points[after_min].x);
                }

                if points[before_max].y > y {
                    max = Some(points[before_max].x);
                } else {
                    max = Some(points[after_max].x);
                }
                let min = min.unwrap();
                let max = max.unwrap();
                *val = (min, max);
            }
            continue;
        }
        if let Some(act_min_x) = min_x {
            if let Some(act_min) = min
                && let Some(found) = y_points.iter().find(|point| point.1.x == act_min)
            {
                min_x = Some(*found);
            }
            if y_points[0].1.x <= act_min_x.1.x {
                min_x = Some(y_points[0]);
            }
        } else {
            min_x = Some(y_points[0]);
        }
        if let Some(act_max_x) = max_x {
            if let Some(act_max) = max
                && let Some(found) = y_points.iter().find(|point| point.1.x == act_max)
            {
                max_x = Some(*found);
            }
            if y_points[y_points.len() - 1].1.x >= act_max_x.1.x {
                max_x = Some(y_points[y_points.len() - 1]);
            }
        } else {
            max_x = Some(y_points[y_points.len() - 1]);
        }
        let act_min_x = min_x.unwrap();
        let act_max_x = max_x.unwrap();
        *val = (act_min_x.1.x, act_max_x.1.x);
        min = None;
        max = None;
    }

    let mut max = 0;
    let mut other_points = Vec::new();
    for point1 in points {
        'innerPoint: for point2 in other_points.iter() {
            let sub = point1.abs_diff(point2);
            let area = (sub.x + 1) * (sub.y + 1);
            let min_x = point1.x.min(point2.x);
            let max_x = point1.x.max(point2.x);
            let min_y = point1.y.min(point2.y);
            let max_y = point1.y.max(point2.y);
            for (min, max) in green.iter().take(max_y + 1).skip(min_y) {
                if *min > min_x || *max < max_x {
                    continue 'innerPoint;
                }
            }
            max = max.max(area);
        }
        other_points.push(point1);
    }

    max.to_string().to_owned()
}
