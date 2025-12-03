struct Max {
    i: usize,
    j: usize,
    val: u32,
}
struct MaxTwelve {
    i1: usize,
    i2: usize,
    i3: usize,
    i4: usize,
    i5: usize,
    i6: usize,
    i7: usize,
    i8: usize,
    i9: usize,
    i10: usize,
    i11: usize,
    i12: usize,
    val: u32,
}

fn find_two_largest(bank: Vec<u32>) -> Max {
    let mut max = Max { i: 0, j: 0, val: 0 };
    for (i, val1) in bank.iter().enumerate() {
        for (j, val2) in bank.iter().enumerate() {
            if i >= j {
                continue;
            }
            if val1 * 10 + val2 > max.val {
                max.i = i;
                max.j = j;
                max.val = val1 * 10 + val2;
            }
        }
    }
    max
}

fn find_twelve_recursive(depth: u8, bank: &[u64]) -> u64 {
    if depth == 0 || bank.len() < depth.into() {
        return 0;
    }
    let mut bank_iter = bank.iter();
    let &val = bank_iter.next().unwrap();
    let new_bank = bank_iter.copied().collect::<Vec<_>>();
    if bank.len() > depth.into()
        && val
            < new_bank
                .iter()
                .enumerate()
                .fold(0_u64, |acc: u64, (i, val2)| {
                    if new_bank.len() - i >= depth.into() {
                        acc.max(*val2)
                    } else {
                        acc
                    }
                })
    {
        return find_twelve_recursive(depth, &new_bank);
    }
    let new_val =
        val * (10_u64).pow((depth - 1).into()) + find_twelve_recursive(depth - 1, &new_bank);
    let new_val2 = find_twelve_recursive(depth, &new_bank);
    new_val.max(new_val2)
}

// Brute force, doesn't work
fn find_twelve(bank: Vec<u64>) -> u64 {
    let mut max = 0;
    for (i1, val1) in bank.iter().enumerate() {
        for (i2, val2) in bank.iter().enumerate() {
            if i1 >= i2 {
                continue;
            }
            for (i3, val3) in bank.iter().enumerate() {
                if i2 >= i3 {
                    continue;
                }
                for (i4, val4) in bank.iter().enumerate() {
                    if i3 >= i4 {
                        continue;
                    }
                    for (i5, val5) in bank.iter().enumerate() {
                        if i4 >= i5 {
                            continue;
                        }
                        for (i6, val6) in bank.iter().enumerate() {
                            if i5 >= i6 {
                                continue;
                            }
                            for (i7, val7) in bank.iter().enumerate() {
                                if i6 >= i7 {
                                    continue;
                                }
                                for (i8, val8) in bank.iter().enumerate() {
                                    if i7 >= i8 {
                                        continue;
                                    }
                                    for (i9, val9) in bank.iter().enumerate() {
                                        if i8 >= i9 {
                                            continue;
                                        }
                                        for (i10, val10) in bank.iter().enumerate() {
                                            if i9 >= i10 {
                                                continue;
                                            }
                                            for (i11, val11) in bank.iter().enumerate() {
                                                if i10 >= i11 {
                                                    continue;
                                                }
                                                for (i12, val12) in bank.iter().enumerate() {
                                                    if i11 >= i12 {
                                                        continue;
                                                    }
                                                    let act_val = val1 * 100000000000
                                                        + val2 * 10000000000
                                                        + val3 * 1000000000
                                                        + val4 * 100000000
                                                        + val5 * 10000000
                                                        + val6 * 1000000
                                                        + val7 * 100000
                                                        + val8 * 10000
                                                        + val9 * 1000
                                                        + val10 * 100
                                                        + val11 * 10
                                                        + val12;
                                                    if act_val > max {
                                                        max = act_val;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    max
}

pub fn part1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let bank = line
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        res += find_two_largest(bank).val;
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
        res += find_twelve_recursive(12, &bank);
        println!("Found number");
    }
    res.to_string().to_owned()
}
