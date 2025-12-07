use std::{collections::HashSet, iter::FilterMap};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Beacon {
    x: usize,
    amount: usize,
}

pub fn part1(input: &str) -> String {
    let mut beacons = HashSet::new();
    let mut res = 0;
    for (y, line) in input.lines().enumerate() {
        // println!("{y}");
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                beacons.insert(x);
            } else if char == '^' {
                let mut new_beacons = beacons.clone();
                let remove_beacons = beacons
                    .iter()
                    .filter(|&&beacon| beacon == x)
                    .collect::<Vec<_>>();
                // println!("{:?}", remove_beacons);
                res += remove_beacons.len();
                for beacon in remove_beacons.iter() {
                    new_beacons.remove(beacon);
                    new_beacons.insert(*beacon - 1);
                    new_beacons.insert(*beacon + 1);
                }
                beacons = new_beacons;
            }
        }
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut beacons = Vec::new();
    for (y, line) in input.lines().enumerate() {
        // println!("{y}");
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                beacons.push(Beacon { x, amount: 1 });
            } else if char == '^' {
                let mut new_beacons = Vec::new();
                let remove_beacons = beacons
                    .iter()
                    .enumerate()
                    .filter(|(_, beacon)| beacon.x == x)
                    .collect::<Vec<_>>();
                // println!("remove_beacons: {:?}", remove_beacons);
                for (_, beacon) in remove_beacons.iter().rev() {
                    if let Some(before) = beacons.iter().find(|beacon2| beacon2.x == beacon.x - 1) {
                        new_beacons.push(Beacon {
                            x: before.x,
                            amount: before.amount + beacon.amount,
                        });
                    } else {
                        new_beacons.push(Beacon {
                            x: beacon.x - 1,
                            amount: beacon.amount,
                        });
                    }

                    if let Some(after) = beacons.iter().find(|beacon2| beacon2.x == beacon.x + 1) {
                        new_beacons.push(Beacon {
                            x: after.x,
                            amount: after.amount + beacon.amount,
                        });
                    } else {
                        new_beacons.push(Beacon {
                            x: beacon.x + 1,
                            amount: beacon.amount,
                        });
                    }
                }
                // println!("new_beacons {:?}", new_beacons);
                let new_new_beacons = new_beacons
                    .iter()
                    .chain(beacons.iter().filter(|beacon| {
                        remove_beacons
                            .iter()
                            .filter(|(_, beacon2)| {
                                beacon2.x - 1 == beacon.x
                                    || beacon2.x + 1 == beacon.x
                                    || beacon2.x == beacon.x
                            })
                            .count()
                            == 0
                    }))
                    .copied()
                    .collect();
                beacons = new_new_beacons;
            }
            // println!("{:?}", beacons);
        }
    }
    // println!("{:?}", beacons);

    beacons
        .iter()
        .map(|beacon| beacon.amount)
        .sum::<usize>()
        .to_string()
        .to_owned()
}
