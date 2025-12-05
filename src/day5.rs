#[derive(Clone, Copy, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

pub fn part1(input: &str) -> String {
    let mut first_parsing = true;
    let mut ranges = Vec::new();
    let mut query = Vec::new();
    for line in input.lines() {
        if first_parsing {
            if line.is_empty() {
                first_parsing = false;
                continue;
            }
            let (start, end) = line.split_once("-").unwrap();
            ranges.push(Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            });
        } else {
            query.push(line.parse().unwrap());
        }
    }

    query
        .into_iter()
        .filter(|&val| {
            ranges
                .iter()
                .any(|range| range.start <= val && range.end >= val)
        })
        .count()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    let mut ranges = Vec::new();
    for line in input.lines().take_while(|line| !line.is_empty()) {
        let (start, end) = line.split_once("-").unwrap();
        ranges.push(Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        });
    }

    let mut new_ranges = ranges.clone();
    while {
        ranges = new_ranges.clone();
        new_ranges = Vec::new();
        let mut skip_indices = Vec::new();
        'range: for (i, range) in ranges.iter().enumerate() {
            for (j, other_range) in ranges.iter().enumerate() {
                if i == j || skip_indices.contains(&j) {
                    continue;
                }
                let mut change = false;
                if range.start >= other_range.start && range.start <= other_range.end {
                    if range.end > other_range.end {
                        new_ranges.push(Range {
                            start: other_range.end + 1,
                            end: range.end,
                        });
                    }
                    change = true;
                }
                if range.end >= other_range.start && range.end <= other_range.end {
                    if range.start < other_range.start {
                        new_ranges.push(Range {
                            start: range.start,
                            end: other_range.start - 1,
                        });
                    }
                    change = true;
                }
                if change {
                    skip_indices.push(i);
                    continue 'range;
                }
            }
            new_ranges.push(*range);
        }
        new_ranges != ranges
    } {}

    new_ranges
        .into_iter()
        .fold(0, |acc, range| acc + range.end - range.start + 1)
        .to_string()
        .to_owned()
}
