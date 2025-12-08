use core::fmt;

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    connected: Vec<Point>,
}

impl Point {
    fn euclid(self, other: Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

#[derive(Clone, Copy)]
struct Dist {
    dist: f32,
    circuit_index: usize,
    circuit2_index: usize,
    point_index: usize,
    point2_index: usize,
}

pub fn part1(input: &str) -> String {
    let mut circuits: Vec<Vec<Point>> = vec![Vec::new(); input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let mut split = line.split(',');
        let point = Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
            connected: Vec::new(),
        };
        circuits[i].push(point.clone());
    }

    let max = if circuits.len() >= 100 { 1000 } else { 10 }; // For test input

    for _ in 0..max {
        // println!("{i}");
        let mut min: Option<Dist> = None;
        for (j, circuit) in circuits.iter().enumerate() {
            for (point_index, point) in circuit.iter().enumerate() {
                for (k, circuit2) in circuits.iter().enumerate() {
                    if j > k {
                        continue;
                    }
                    for (point2_index, point2) in circuit2.iter().enumerate() {
                        if point2.connected.contains(point) || point.connected.contains(point2) {
                            continue;
                        }

                        if j == k && point_index == point2_index {
                            continue;
                        }

                        let dist = point.clone().euclid(point2.clone());
                        if let Some(act_min) = min {
                            if act_min.dist > dist {
                                min = Some(Dist {
                                    dist,
                                    circuit_index: j,
                                    circuit2_index: k,
                                    point_index,
                                    point2_index,
                                });
                            }
                        } else {
                            min = Some(Dist {
                                dist,
                                circuit_index: j,
                                circuit2_index: k,
                                point_index,
                                point2_index,
                            });
                        }
                    }
                }
            }
        }
        let min = min.unwrap();
        let mut new_point2 = circuits[min.circuit2_index][min.point2_index].clone();
        // println!(
        // "comparing {:?} with {:?}",
        // circuits[min.circuit_index][min.point_index], new_point2
        // );
        circuits[min.circuit_index][min.point_index]
            .connected
            .push(new_point2.clone());
        new_point2
            .connected
            .push(circuits[min.circuit_index][min.point_index].clone());
        if min.circuit_index != min.circuit2_index {
            for point2 in circuits.remove(min.circuit2_index) {
                circuits[min.circuit_index].push(point2.clone());
            }
        } else {
            // println!();
            // println!("skip");
        }
        // println!();
        // println!("{i}: {:?}", circuits);
    }

    circuits.sort_unstable_by_key(|points2| std::cmp::Reverse(points2.len()));
    // println!();
    // println!("result: {:?}", circuits);

    circuits
        .iter()
        .take(3)
        .map(|points| points.len() as u64)
        .reduce(|acc, length| acc * length)
        .unwrap()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let mut circuits: Vec<Vec<Point>> = vec![Vec::new(); input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let mut split = line.split(',');
        let point = Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
            connected: Vec::new(),
        };
        circuits[i].push(point.clone());
    }

    let mut last = (None, None);

    while circuits.len() != 1 {
        let mut min: Option<Dist> = None;
        for (j, circuit) in circuits.iter().enumerate() {
            for (point_index, point) in circuit.iter().enumerate() {
                for (k, circuit2) in circuits.iter().enumerate() {
                    if j >= k {
                        continue;
                    }
                    for (point2_index, point2) in circuit2.iter().enumerate() {
                        let dist = point.clone().euclid(point2.clone());
                        if let Some(act_min) = min {
                            if act_min.dist > dist {
                                min = Some(Dist {
                                    dist,
                                    circuit_index: j,
                                    circuit2_index: k,
                                    point_index,
                                    point2_index,
                                });
                            }
                        } else {
                            min = Some(Dist {
                                dist,
                                circuit_index: j,
                                circuit2_index: k,
                                point_index,
                                point2_index,
                            });
                        }
                    }
                }
            }
        }
        let min = min.unwrap();
        let mut new_point2 = circuits[min.circuit2_index][min.point2_index].clone();
        last = (
            Some(circuits[min.circuit_index][min.point_index].x),
            Some(new_point2.x),
        );
        // println!(
        // "comparing {:?} with {:?}",
        // circuits[min.circuit_index][min.point_index], new_point2
        // );
        circuits[min.circuit_index][min.point_index]
            .connected
            .push(new_point2.clone());
        new_point2
            .connected
            .push(circuits[min.circuit_index][min.point_index].clone());
        if min.circuit_index != min.circuit2_index {
            for point2 in circuits.remove(min.circuit2_index) {
                circuits[min.circuit_index].push(point2.clone());
            }
        } else {
            // println!();
            // println!("skip");
        }
        // println!();
        // println!("{i}: {:?}", circuits);
    }

    circuits.sort_unstable_by_key(|points2| std::cmp::Reverse(points2.len()));
    // println!();
    // println!("result: {:?}", circuits);

    (last.0.unwrap() * last.1.unwrap()).to_string().to_owned()
}
