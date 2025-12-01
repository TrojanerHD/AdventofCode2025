pub fn part1(input: &str) -> String {
    let mut pos: i16 = 50;
    let mut count = 0;
    for line in input.lines() {
        if line.starts_with("R") {
            let val = line.split_once("R").unwrap().1.parse::<i16>().unwrap();
            pos = (pos + val) % 100;
        } else {
            let val = line.split_once("L").unwrap().1.parse::<i16>().unwrap();
            pos = (pos - val) % 100;
        }
        if pos == 0 {
            count += 1;
        }
    }
    count.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut pos: i16 = 50;
    let mut count = 0;
    for line in input.lines() {
        if line.starts_with("R") {
            let val = line.split_once("R").unwrap().1.parse::<i16>().unwrap();
            for _ in 0..val {
                pos = (pos + 1) % 100;
                if pos == 0 {
                    count += 1;
                }
            }
        } else {
            let val = line.split_once("L").unwrap().1.parse::<i16>().unwrap();
            for _ in 0..val {
                pos = (pos - 1) % 100;
                if pos == 0 {
                    count += 1;
                }
            }
        }
    }
    count.to_string().to_owned()
}
