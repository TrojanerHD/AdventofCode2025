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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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
        circuits[i].push(point);
    }

    let max = if circuits.len() >= 100 { 1000 } else { 10 }; // For test input

    for _ in 0..max {
        let mut min: Option<Dist> = None;
        for (j, circuit) in circuits.iter().enumerate() {
            for (k, circuit2) in circuits.iter().enumerate().skip(j) {
                for (point_index, point) in circuit.iter().enumerate() {
                    for (point2_index, point2) in circuit2.iter().enumerate() {
                        if point2.connected.contains(point)
                            || point.connected.contains(point2)
                            || (j == k && point_index == point2_index)
                        {
                            continue;
                        }

                        let euclid = point.clone().euclid(point2.clone());
                        let dist = Dist {
                            dist: euclid,
                            circuit_index: j,
                            circuit2_index: k,
                            point_index,
                            point2_index,
                        };
                        if let Some(act_min) = min {
                            if act_min > dist {
                                min = Some(dist);
                            }
                        } else {
                            min = Some(dist);
                        }
                    }
                }
            }
        }
        let min = min.unwrap();
        let mut new_point2 = circuits[min.circuit2_index][min.point2_index].clone();
        circuits[min.circuit_index][min.point_index]
            .connected
            .push(new_point2.clone());
        new_point2
            .connected
            .push(circuits[min.circuit_index][min.point_index].clone());
        if min.circuit_index != min.circuit2_index {
            let to_add = circuits.swap_remove(min.circuit2_index);
            circuits[min.circuit_index].extend(to_add);
        }
    }

    let mut circuit_lengths = circuits
        .iter()
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();

    circuit_lengths.sort_unstable();

    circuit_lengths
        .iter()
        .rev()
        .take(3)
        .copied()
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
        circuits[i].push(point);
    }

    let mut last = 0;

    while circuits.len() != 1 {
        let mut min: Option<Dist> = None;
        for (j, circuit) in circuits.iter().enumerate() {
            for (k, circuit2) in circuits.iter().enumerate().skip(j + 1) {
                for (point_index, point) in circuit.iter().enumerate() {
                    for (point2_index, point2) in circuit2.iter().enumerate() {
                        let euclid = point.clone().euclid(point2.clone());
                        let dist = Dist {
                            dist: euclid,
                            circuit_index: j,
                            circuit2_index: k,
                            point_index,
                            point2_index,
                        };
                        if let Some(act_min) = min {
                            if act_min > dist {
                                min = Some(dist);
                            }
                        } else {
                            min = Some(dist);
                        }
                    }
                }
            }
        }

        let min = min.unwrap();
        last = circuits[min.circuit_index][min.point_index].x
            * circuits[min.circuit2_index][min.point2_index].x;
        let to_add = circuits.swap_remove(min.circuit2_index);
        circuits[min.circuit_index].extend(to_add);
    }

    last.to_string().to_owned()
}
