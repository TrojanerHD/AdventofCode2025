use std::usize;

#[derive(Debug, Clone, Copy)]
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
    let mut points = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        points.push(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        });
    }

    let mut max = 0;
    for point1 in points.iter() {
        for point2 in points.iter() {
            if point2.y <= point1.y && point2.x <= point1.x {
                continue;
            }
            let sub = point1.abs_diff(point2);
            let area = (sub.x + 1) * (sub.y + 1);
            max = max.max(area);
        }
    }

    max.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut points = Vec::new();
    let mut y_max = 0_usize;
    let mut x_max = 0_usize;
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        points.push(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        });
        x_max = x_max.max(x.parse().unwrap());
        y_max = y_max.max(y.parse().unwrap());
    }

    let mut min_x: Option<(usize, Point)> = None;
    let mut max_x: Option<(usize, Point)> = None;
    let mut green = vec![(0, 0); y_max + 1];
    let mut min = None;
    let mut max = None;
    for y in 0..=y_max {
        // println!("y {y}");
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
                min = None;
                max = None;
                let mut before_min = wrapping(min_x.0, true, points.len());
                let mut after_min = wrapping(min_x.0, false, points.len());
                let mut before_max = wrapping(max_x.0, true, points.len());
                let mut after_max = wrapping(max_x.0, false, points.len());

                while points[before_min].y <= y && points[after_min].y <= y {
                    before_min = wrapping(before_min, true, points.len());
                    after_min = wrapping(after_min, false, points.len());
                    // println!(
                    //     "before_min {before_min} for {} with {:?}, after_min {after_min} for {} with {:?}",
                    //     min_x.0, points[before_min], max_x.0, points[after_min]
                    // );
                }

                while points[before_max].y <= y && points[after_max].y <= y {
                    before_max = wrapping(before_max, true, points.len());
                    after_max = wrapping(after_max, false, points.len());
                }

                if points[before_min].y > y {
                    // println!("min x: {:?}", points[before_min]);
                    min = Some(points[before_min].x);
                }
                if points[after_min].y > y {
                    // println!("min x: {:?}", points[after_min]);
                    min = Some(points[after_min].x);
                }
                if points[before_max].y > y {
                    max = Some(points[before_max].x);
                }
                if points[after_max].y > y {
                    max = Some(points[after_max].x);
                }
                let Some(min) = min else {
                    panic!("min is None and min_x is {:?} at y {y}", min_x);
                };
                let Some(max) = max else {
                    panic!("max is None");
                };

                green[y] = (min, max);
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
            // if let Some(act_min) = min
            //     && let Some(found) = y_points.iter().find(|point| point.1.x == act_min)
            // {
            //     min_x = Some(*found);
            // }
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
            // if let Some(act_max) = max
            //     && let Some(found) = y_points.iter().find(|point| point.1.x == act_max)
            // {
            //     max_x = Some(*found);
            // }
            max_x = Some(y_points[y_points.len() - 1]);
        }
        let act_min_x = min_x.unwrap();
        let act_max_x = max_x.unwrap();
        for x in act_min_x.1.x..=act_max_x.1.x {
            green[y] = (act_min_x.1.x, act_max_x.1.x);
        }
    }

    // println!("green {:?}", green);

    let mut max = (0, (&Point { x: 0, y: 0 }, &Point { x: 0, y: 0 }));
    for point1 in points.iter() {
        // println!("{i} / {}", points.len());
        'innerPoint: for point2 in points.iter() {
            if point2.y <= point1.y && point2.x <= point1.x {
                continue;
            }
            let sub = point1.abs_diff(point2);
            let area = (sub.x + 1) * (sub.y + 1);
            let min_x = point1.x.min(point2.x);
            let max_x = point1.x.max(point2.x);
            let min_y = point1.y.min(point2.y);
            let max_y = point1.y.max(point2.y);
            for y in min_y..=max_y {
                if green[y].0 > min_x || green[y].1 < max_x {
                    continue 'innerPoint;
                }
            }
            max = if max.0 < area {
                (area, (point1, point2))
            } else {
                max
            }
        }
    }

    // println!("{:?}", max);

    max.0.to_string().to_owned()
}
