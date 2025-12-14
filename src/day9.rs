use std::collections::HashMap;

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
        (index + 1) % len
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Point2 {
    point: Point,
    index: Option<usize>,
}

impl Point2 {
    fn new(x: usize, y: usize, index: Option<usize>) -> Self {
        Self {
            point: Point { x, y },
            index,
        }
    }
    fn x(&self) -> usize {
        self.point.x
    }
    fn y(&self) -> usize {
        self.point.y
    }
}

fn calc_bounds(
    points: &[Point2],
    hash_points: &HashMap<usize, Vec<(usize, Option<usize>)>>,
    y: usize,
) -> Vec<(usize, usize)> {
    let mut new_bounds = Vec::new();
    let mut min = None;
    let mut take = false;
    let y_points = hash_points.get(&y).unwrap();
    for (j, point) in y_points.iter().enumerate() {
        let prev_take = take;
        if let Some(i) = point.1 {
            let next_point = &points[wrapping(i, false, points.len())];
            let next_y_point = y_points.get(j + 1);
            take = if !take {
                true
            } else {
                next_point.y() == y || next_y_point.is_some_and(|point2| point2.1.is_none())
            };
        } else {
            take = !take;
        }
        if !prev_take && take {
            min = Some(point.0);
        } else if prev_take && !take {
            new_bounds.push((min.unwrap(), point.0));
            min = None;
        }
    }
    new_bounds
}

pub fn part2(input: &str) -> String {
    let mut y_min = usize::MAX;
    let mut x_min = usize::MAX;
    let mut y_max = 0;
    let mut x_max = 0;
    let mut hash_points: HashMap<usize, Vec<(usize, Option<usize>)>> = HashMap::new();
    let points = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (x, y) = line.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            y_max = y_max.max(y);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            x_min = x_min.min(x);
            hash_points
                .entry(y)
                .and_modify(|vec| vec.push((x, Some(i))))
                .or_insert(vec![(x, Some(i))]);
            Point2::new(x, y, Some(i))
        })
        .collect::<Vec<_>>();

    for (i, point) in points.iter().enumerate() {
        for j in [
            wrapping(i, true, points.len()),
            wrapping(i, false, points.len()),
        ] {
            let point2 = &points[j];
            if point <= point2 {
                continue;
            }
            if point.x() == point2.x() {
                for y in (point.y().min(point2.y()) + 1)..point.y().max(point2.y()) {
                    hash_points
                        .entry(y)
                        .and_modify(|vec| vec.push((point.x(), None)))
                        .or_insert(vec![(point.x(), None)]);
                }
            }
        }
    }

    let mut greens = vec![None; y_max + 1];

    for y_points in hash_points.values_mut() {
        y_points.sort_unstable_by_key(|point| point.0);
    }
    let mut max = 0;
    for (i, point1) in points.iter().enumerate() {
        'innerPoint: for point2 in points.iter().take(i - 1) {
            let min_x = point1.x().min(point2.x());
            let max_x = point1.x().max(point2.x());
            let min_y = point1.y().min(point2.y());
            let max_y = point1.y().max(point2.y());

            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            if area <= max {
                continue;
            }
            for (y, bounds) in greens.iter_mut().enumerate().take(max_y + 1).skip(min_y) {
                if bounds.is_none() {
                    *bounds = Some(calc_bounds(&points, &hash_points, y));
                }
                if !bounds
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|(min, max)| *min <= min_x && *max >= max_x)
                {
                    continue 'innerPoint;
                }
            }

            max = area;
        }
    }

    max.to_string().to_owned()
}
