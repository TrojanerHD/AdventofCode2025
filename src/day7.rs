use std::collections::HashSet;

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
    let mut beacons = vec![0_u64; input.lines().next().unwrap().len()];
    for line in input.lines() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                beacons.insert(x, 1);
            } else if char == '^' && beacons[x] != 0 {
                beacons[x - 1] += beacons[x];
                beacons[x + 1] += beacons[x];
                beacons[x] = 0;
            }
        }
    }

    beacons.into_iter().sum::<u64>().to_string().to_owned()
}
