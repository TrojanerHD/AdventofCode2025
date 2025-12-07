use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> String {
    let mut beacons = HashSet::new();
    let mut res = 0;
    for line in input.lines() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                beacons.insert(x);
            } else if char == '^' {
                let mut new_beacons = beacons.clone();
                let remove_beacons = beacons.get(&x);
                if let Some(beacon) = remove_beacons {
                    res += 1;
                    new_beacons.remove(beacon);
                    new_beacons.insert(*beacon - 1);
                    new_beacons.insert(*beacon + 1);
                    beacons = new_beacons;
                }
            }
        }
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut beacons = HashMap::new();
    for line in input.lines() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                beacons.insert(x, 1);
            } else if char == '^' && let Some(amount) = beacons.remove(&x) {
                beacons
                    .entry(x - 1)
                    .and_modify(|before| *before += amount)
                    .or_insert(amount);

                beacons
                    .entry(x + 1)
                    .and_modify(|after| *after += amount)
                    .or_insert(amount);
                }
        }
    }

    beacons.values().sum::<u64>().to_string().to_owned()
}
