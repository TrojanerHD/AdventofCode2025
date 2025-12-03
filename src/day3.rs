fn find_largest(depth: u8, bank: &[u64]) -> u64 {
    if depth == 0 || bank.len() < depth.into() {
        return 0;
    }
    let mut bank_iter = bank.iter();
    let &val = bank_iter.next().unwrap();
    let new_bank = bank_iter.copied().collect::<Vec<_>>();
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
    let mut res = 0;
    for line in input.lines() {
        let bank = line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();
        res += find_largest(2, &bank);
    }
    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let bank = line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();
        res += find_largest(12, &bank);
    }
    res.to_string().to_owned()
}
