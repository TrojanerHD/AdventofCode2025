pub fn part1(input: &str) -> String {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse::<usize>().unwrap()..=end.parse().unwrap()).map(|count| {
                let count_string = count.to_string();
                if count_string.len() % 2 == 0 {
                    let split = count_string.split_at(count_string.len() / 2);
                    if split.0 == split.1 {
                        return count;
                    }
                }
                0
            })
        })
        .sum::<usize>()
        .to_string()
        .to_owned()
}

fn split_at_every(input: &str) -> usize {
    (1..=input.len() / 2)
        .filter(|&i| input.len().is_multiple_of(i))
        .find_map(|i| {
            let (truth, mut rest) = input.split_at(i);
            while !rest.is_empty() {
                let split_n = rest.split_at(i);
                if truth != split_n.0 {
                    return None;
                }
                rest = split_n.1;
            }
            Some(input.parse::<usize>().unwrap())
        })
        .unwrap_or(0)
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse::<usize>().unwrap()..=end.parse().unwrap())
                .map(|count| split_at_every(&count.to_string()))
        })
        .sum::<usize>()
        .to_string()
        .to_owned()
}
