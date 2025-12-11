use std::time::Instant;

fn find_shortest_pattern(goal: Vec<bool>, buttons: Vec<Vec<u16>>) -> u16 {
    if !goal.contains(&true) {
        return 0;
    }
    let mut possible_combinations = Vec::new();
    let mut patterns = vec![(vec![false; goal.len()], 0)];
    loop {
        let mut new_patterns = patterns.clone();
        for (pattern, button_presses) in patterns.iter() {
            for button in buttons.iter() {
                let changed_pattern = change_button(pattern.clone(), button.clone());
                if let Some(existing_pattern) = new_patterns
                    .iter_mut()
                    .find(|(pattern, _)| *pattern == changed_pattern)
                {
                    existing_pattern.1 = existing_pattern.1.min(button_presses + 1);
                } else {
                    new_patterns.push((changed_pattern.clone(), button_presses + 1));
                }
                if changed_pattern == goal {
                    possible_combinations.push(button_presses + 1);
                }
            }
        }

        if new_patterns.len() == patterns.len() {
            return *possible_combinations.iter().min().unwrap();
        }
        patterns = new_patterns;
    }
}

fn combinations(length: usize, total: u16) -> impl Iterator<Item = Vec<u16>> {
    let mut combination = vec![0; length];
    combination[0] = total;
    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }
        if length == 1 {
            done = true;
            return Some(combination.clone());
        }
        loop {
            if combination[length - 1] == total {
                done = true;
            }
            let prev = combination.clone();
            let res = prev.iter().sum();
            let mut change = false;
            if combination[0] == 0 {
                if length >= 2 {
                    if combination[1] == 0 {
                        change = true;
                        *combination.iter_mut().find(|val| **val != 0).unwrap() = total + 1;
                    } else {
                        combination.swap(0, 1);
                    }
                    if combination[0] > total {
                        return None;
                    }
                } else {
                    done = true;
                }
                if length >= 3 && !change {
                    combination[2] += 1;
                }
            } else {
                combination[1] += 1;
            }
            for i in 1..length {
                if combination[i] > total {
                    if i == length - 1 {
                        done = true;
                        break;
                    }
                    change = true;
                    combination[i + 1] += 1;
                    combination[i] = 0;
                    combination[0] = total - combination.iter().sum::<u16>();
                    break;
                }
            }
            if !change {
                combination[0] -= 1;
            }
            if total == res {
                if length == 1 {
                    done = true;
                }
                return Some(prev);
            }
        }
    })
}

fn recursive2(pattern: &[u16], buttons: &[Vec<u16>]) -> Option<u16> {
    if pattern.iter().all(|val| *val == 0) {
        return Some(0);
    }

    let no_zeroes = pattern
        .iter()
        .enumerate()
        .filter(|(_, val)| **val != 0)
        .map(|(i, val)| {
            (
                i,
                val,
                buttons
                    .iter()
                    .filter(|button| button.contains(&(i as u16)))
                    .count(),
            )
        })
        .collect::<Vec<_>>();

    if buttons.is_empty() || no_zeroes.iter().any(|(_, _, count)| *count == 0) {
        return None;
    }

    let (_, _, reduced_btn_cnt) = no_zeroes
        .iter()
        .min_by_key(|(_, _, btn_cnt)| btn_cnt)
        .unwrap();

    let (i, to_reduce, _) = no_zeroes
        .iter()
        .filter(|(_, _, btn_cnt)| btn_cnt == reduced_btn_cnt)
        .min_by_key(|(val, _, _)| *val)
        .unwrap();

    let (reduced_buttons, new_buttons): (Vec<_>, Vec<_>) = buttons
        .iter()
        .cloned()
        .partition(|button| button.contains(&(*i as u16)));

    if reduced_buttons.is_empty() {
        return None;
    }

    let mut min = None;
    'combinations: for combination in combinations(reduced_buttons.len(), **to_reduce) {
        let mut new_pattern = pattern.to_vec().clone();
        for (i, val) in combination.iter().enumerate() {
            let Some(pat) = change_button_2_val(new_pattern, &reduced_buttons[i], val) else {
                continue 'combinations;
            };
            new_pattern = pat;
        }

        if let Some(recurse) = recursive2(&new_pattern, &new_buttons)
            && min.is_none_or(|min| min > recurse + *to_reduce)
        {
            min = Some(recurse + *to_reduce);
        }
    }
    min
}

fn change_button(mut pattern: Vec<bool>, button: Vec<u16>) -> Vec<bool> {
    for press in button {
        pattern[press as usize] = !pattern[press as usize];
    }
    pattern
}

fn change_button_2_val(mut pattern: Vec<u16>, button: &[u16], amount: &u16) -> Option<Vec<u16>> {
    for &press in button {
        let new_val = pattern[press as usize].checked_sub(*amount);
        if let Some(new_val) = new_val {
            pattern[press as usize] = new_val;
        } else {
            return None;
        }
    }
    Some(pattern)
}

pub fn part1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut all_inputs = line.split_whitespace();
        let goal = all_inputs
            .next()
            .unwrap()
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|char| char == '#')
            .collect::<Vec<_>>();

        let _ = all_inputs.next_back();
        let buttons = all_inputs
            .map(|button| {
                button
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|press| press.parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        res += find_shortest_pattern(goal, buttons);
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let start = Instant::now();
    let mut res = 0;
    for (i, line) in input.lines().enumerate() {
        let start1 = Instant::now();
        let mut all_inputs = line.split_whitespace();
        let _ = all_inputs.next();
        let joltage = all_inputs
            .next_back()
            .unwrap()
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .map(|a| a.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        let mut buttons = all_inputs
            .map(|button| {
                button
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|press| press.parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        buttons.sort_unstable_by_key(|button| std::cmp::Reverse(button.len()));
        let val = recursive2(&joltage, &buttons).unwrap();
        res += val;
        println!(
            "Solved {} / {} with {val}. Took: {:?}",
            i + 1,
            input.lines().count(),
            start1.elapsed()
        );
    }

    println!("Took {:?}", start.elapsed());

    res.to_string().to_owned()
}
