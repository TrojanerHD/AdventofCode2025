#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    start: usize,
    end: usize,
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
            query.push(line.parse::<usize>().unwrap());
        }
    }

    let mut res = 0;
    for val in query {
        if ranges
            .iter()
            .any(|range| range.start <= val && range.end >= val)
        {
            res += 1;
        }
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut ranges = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once("-").unwrap();
        ranges.push(Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        });
    }

    //
    // let val = ranges
    //     .iter()
    //     .flat_map(|range| (range.start..=range.end).collect::<Vec<usize>>())
    //     .collect::<Vec<_>>();
    // println!("{:?}", val);
    // val.iter().dedup().count().to_string().to_owned()
    //
    // let mut all_vals = HashSet::new();
    // for range in ranges.iter() {
    //     for val in range.start..=range.end {
    //         all_vals.insert(val);
    //     }
    //     println!("{}, {}", range.start, range.end);
    // }

    let mut new_ranges = ranges.clone();
    let mut first = true;
    while first || new_ranges != ranges {
        first = false;
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
    }
    // for range in new_ranges.iter() {
    // println!("{}-{}", range.start, range.end);
    // }
    new_ranges
        .iter()
        .fold(0, |acc, range| acc + range.end - range.start + 1)
        .to_string()
        .to_owned()
}
