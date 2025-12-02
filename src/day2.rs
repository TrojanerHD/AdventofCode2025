struct Range {
    start: usize,
    end: usize,
}
pub fn part1(input: &str) -> String {
    let ranges = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for range in ranges {
        let mut count = range.start;
        while count <= range.end {
            let count_string = count.to_string();
            if count_string.len() % 2 == 0 {
                let split = count_string.split_at(count_string.len() / 2);
                if split.0 == split.1 {
                    res += count;
                }
            }
            count += 1;
        }
    }

    res.to_string().to_owned()
}

fn split_at_every(input: &str) -> usize {
    'outer: for i in 1..=input.len() / 2 {
        let split = input.split_at(i);
        let truth = split.0;
        let mut rest = split.1;
        while rest.len() >= i {
            let split_n = rest.split_at(i);
            if truth != split_n.0 {
                continue 'outer;
            }
            rest = split_n.1;
        }
        if rest.is_empty() || rest == truth {
            return input.parse::<usize>().unwrap();
        }
    }
    0
}

pub fn part2(input: &str) -> String {
    let ranges = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for range in ranges {
        let mut count = range.start;
        while count <= range.end {
            let count_string = count.to_string();
            res += split_at_every(&count_string);
            count += 1;
        }
    }

    res.to_string().to_owned()
}
