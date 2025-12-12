#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone)]
struct Request {
    x_size: u16,
    y_size: u16,
    shape_indices: Vec<usize>,
}

fn flip(shape: Vec<Point>) -> Vec<Vec<Point>> {
    let mut res = vec![shape.clone()];
    let mut new_shape = Vec::new();
    let point_max = shape.iter().max_by_key(|point| point.y).unwrap().y;
    for point in shape.iter() {
        new_shape.push(Point {
            x: point.x,
            y: point_max - point.y,
        });
    }
    res.push(new_shape);
    new_shape = Vec::new();
    for point in shape.iter() {
        new_shape.push(Point {
            x: point_max - point.x,
            y: point.y,
        });
    }
    res.push(new_shape);
    new_shape = Vec::new();
    for point in shape {
        new_shape.push(Point {
            x: point_max - point.x,
            y: point_max - point.y,
        });
    }
    res.push(new_shape);
    res
}

fn rotate_and_flip(shape: Vec<Point>) -> Vec<Vec<Point>> {
    let shapes = flip(shape);
    let mut res = shapes.clone();
    for shape in shapes {
        let point_max = shape.iter().max_by_key(|point| point.y).unwrap().y;
        let mut new_shape = Vec::new();
        for point in shape.iter() {
            new_shape.push(Point {
                x: point.y,
                y: point.x,
            });
        }
        res.push(new_shape);
        new_shape = Vec::new();
        for point in shape {
            new_shape.push(Point {
                x: point_max - point.y,
                y: point_max - point.x,
            });
        }
    }
    res.dedup_by(|shape1, shape2| {
        shape1.iter().all(|point| shape2.contains(point))
            && shape2.iter().all(|point| shape1.contains(point))
    });
    res
}

fn try_fit(shapes: &[Vec<Point>], request: &Request, field: Vec<Point>) -> bool {
    if request.shape_indices.iter().all(|val| *val == 0) {
        return true;
    }
    for (i, val) in request.shape_indices.iter().enumerate() {
        if *val == 0 {
            continue;
        }
        let shape = shapes[i].clone();
        let mut new_request = request.clone();
        new_request.shape_indices[i] -= 1;
        let point_max = shape.iter().max_by_key(|point| point.y).unwrap().y;
        for y in 0..(request.y_size - point_max) {
            for x in 0..(request.x_size - point_max) {
                'shape: for shape in rotate_and_flip(shape.clone()) {
                    let mut new_field = field.clone();
                    for mut point in shape {
                        point.x += x;
                        point.y += y;
                        if field.contains(&point) {
                            continue 'shape;
                        }
                        new_field.push(point);
                    }
                    if try_fit(shapes, &new_request, new_field) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn part1(input: &str) -> String {
    let mut shapes = Vec::new();
    let mut shape: Vec<Point> = Vec::new();
    let mut requests = Vec::new();
    let mut y = 0;
    let mut done = false;
    for line in input.lines() {
        if line.contains(":") {
            if line.contains("x") {
                assert_eq!(
                    shape.iter().max_by_key(|point| point.y).unwrap().y,
                    shape.iter().max_by_key(|point| point.x).unwrap().x
                );
                if !done {
                    shapes.push(shape.clone());
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
            if shape.is_empty() {
                continue;
            }
            y = 0;
            assert_eq!(
                shape.iter().max_by_key(|point| point.y).unwrap().y,
                shape.iter().max_by_key(|point| point.x).unwrap().x
            );
            shapes.push(shape);
            shape = Vec::new();
            continue;
        }
        shape.extend(
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char == '#' {
                        Some(Point { x: x as u16, y })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );
        y += 1;
    }

    // for shape in rotate_and_flip(shapes[2].clone()) {
    //     let point_max = shape.iter().max_by_key(|point| point.y).unwrap().y;
    //     for y in 0..=point_max {
    //         for x in 0..=point_max {
    //             if shape.iter().any(|point| point.x == x && point.y == y) {
    //                 print!("#")
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }
    println!("{:?}", requests);
    for shape in shapes.iter() {
        println!("{:?}", shape);
    }

    let mut res = 0;
    for request in requests {
        if request.x_size * request.y_size
            >= shapes
                .iter()
                .enumerate()
                .map(|(i, _)| request.shape_indices[i] as u16 * 9)
                .sum()
        {
            res += 1;
        }
        // if try_fit(&shapes, &request, Vec::new()) {
        // res += 1;
        // }
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    "".to_owned()
}
