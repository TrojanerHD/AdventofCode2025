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
    fn abs_diff(&self, other: &Point2) -> Point {
        self.point.abs_diff(&other.point)
    }
}

pub fn part2(input: &str) -> String {
    let mut y_min = usize::MAX;
    let mut x_min = usize::MAX;
    let mut y_max = 0;
    let mut x_max = 0;
    let mut points = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (x, y) = line.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            y_max = y_max.max(y);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            x_min = x_min.min(x);
            Point2::new(x, y, Some(i))
        })
        .collect::<Vec<_>>();

    let original_points = points.clone();

    let mut new_points = Vec::new();

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
                    new_points.push(Point2::new(point.x(), y, None));
                }
            }
        }
    }
    points.extend(new_points);

    let mut greens = vec![Vec::new(); y_max + 1];

    points.sort_unstable_by(|point, point2| {
        match point.y().cmp(&point2.y()) {
            std::cmp::Ordering::Equal => {}
            a => return a,
        }
        point.x().cmp(&point2.x())
    });
    let mut min = None;
    let mut take = false;
    let mut prev_y = 0;
    for (j, point) in points.iter().enumerate() {
        if prev_y != point.y() {
            min = None;
            take = false;
        }
        prev_y = point.y();
        let prev_take = take;
        if let Some(i) = point.index {
            let next_point = &points[wrapping(i, false, points.len())];
            let next_y_point = points.get(j + 1);
            if !take {
                take = true;
            } else {
                take = next_point.y() == point.y()
                    || next_y_point
                        .is_some_and(|point2| point2.index.is_none() && point2.y() == point.y());
            }
        } else {
            take = !take;
        }
        if !prev_take && take {
            min = Some(point.x());
        } else if prev_take && !take {
            greens[point.y()].push((min.unwrap(), point.x()));
            min = None;
        }
    }

    let mut max = 0;
    for point1 in original_points.iter() {
        'innerPoint: for point2 in original_points.iter() {
            if point1 <= point2 {
                continue;
            }
            let sub = point1.abs_diff(point2);
            let area = (sub.x + 1) * (sub.y + 1);
            let min_x = point1.x().min(point2.x());
            let max_x = point1.x().max(point2.x());
            let min_y = point1.y().min(point2.y());
            let max_y = point1.y().max(point2.y());
            for bounds in greens.iter().take(max_y + 1).skip(min_y) {
                for (min, max) in bounds {
                    if *min > min_x || *max < max_x {
                        continue 'innerPoint;
                    }
                }
            }
            max = max.max(area);
        }
    }

    max.to_string().to_owned()
}
