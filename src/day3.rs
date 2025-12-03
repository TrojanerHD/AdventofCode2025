fn find_largest(depth: u8, bank: &[u64]) -> u64 {
    if depth == 0 {
        return 0;
    }
    let mut bank_iter = bank.iter().copied();
    let val = bank_iter.next().unwrap();
    let new_bank = bank_iter.collect::<Vec<_>>();
    if val
        < new_bank.iter().enumerate().fold(0, |acc, (i, &val2)| {
            if new_bank.len() - i >= depth.into() {
                acc.max(val2)
            } else {
                acc
            }
        })
    {
        find_largest(depth, &new_bank)
    } else {
        val * (10_u64).pow((depth - 1).into()) + find_largest(depth - 1, &new_bank)
    }
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let bank = line
                .chars()
                .map(|char| char.to_digit(10).unwrap().into())
                .collect::<Vec<_>>();
            find_largest(2, &bank)
        })
        .sum::<u64>()
        .to_string()
        .to_owned()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let bank = line
                .chars()
                .map(|char| char.to_digit(10).unwrap().into())
                .collect::<Vec<_>>();
            find_largest(12, &bank)
        })
        .sum::<u64>()
        .to_string()
        .to_owned()
}
