use std::{collections::HashMap, time::Instant, u16};

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

fn divisible(pattern: &Vec<u16>, goal: &Vec<u16>) -> Option<u16> {
    if pattern.contains(&0) {
        return None;
    }
    let guess = goal[0] / pattern[0];
    for (i, val) in pattern.iter().enumerate() {
        let val2 = goal[i];
        if !val2.is_multiple_of(*val) || val2 / val != guess {
            return None;
        }
    }
    println!("divisor {guess}");
    Some(guess)
}

fn combinations(length: usize, total: u16, first: bool) -> impl Iterator<Item = Vec<u16>> {
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
            if res != total {
                return None;
            }
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
                    let sum = combination.iter().sum::<u16>();
                    combination[0] = total - sum;
                    break;
                }
            }
            if !change {
                combination[0] -= 1;
            }
            if total == res {
                if first {
                    // let skipped = [0, 0, 14, 3, 1];
                    // if prev[4] < skipped[4]
                    // || (prev[4] == skipped[4] && prev[3] < skipped[3])
                    // || (prev[4] == skipped[4] && prev[3] == skipped[3] && prev[2] < skipped[2])
                    // && first
                    // {
                    // continue;
                    // }
                    // println!("Trying {:?}", prev);
                }
                if length == 1 {
                    done = true;
                }
                return Some(prev);
            }
        }
    })
}

fn recursive2(pattern: &[u16], buttons: &[Vec<u16>], first: bool) -> Option<u16> {
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
    if first {
        // println!("{to_reduce}");
    }

    // let smallest_btn = buttons.iter().map(|button| pattern).min_by_key(|button| button.len());

    // let (i, to_reduce) = no_zeroes.into_iter().min_by_key(|(_, val)| *val)?;

    let (reduced_buttons, new_buttons): (Vec<_>, Vec<_>) = buttons
        .iter()
        .cloned()
        .partition(|button| button.contains(&(*i as u16)));
    // if [97, 72, 75, 76, 50, 94, 83, 89, 97, 82].contains(to_reduce) {
    // println!("{to_reduce}, {}", reduced_buttons.len());
    // }
    // let reduced_buttons = reduced_buttons
    //     .iter()
    //     .map(|button| {
    //         button
    //             .iter()
    //             .filter_map(|val| {
    //                 if *val > i as u16 {
    //                     Some(val - 1)
    //                 } else if *val < i as u16 {
    //                     Some(*val)
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();
    // let new_buttons = new_buttons
    //     .iter()
    //     .map(|button| {
    //         button
    //             .iter()
    //             .map(|val| if *val > i as u16 { val - 1 } else { *val })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();
    if reduced_buttons.is_empty() {
        // println!("red_btn empty");
        // let val = recursive2(min, pattern, &new_buttons, recursive_reduce + to_reduce)
        //     .map(|val| val + to_reduce);
        // println!("{:?}", val);
        return None;
    }

    let mut min = None;
    'combinations: for combination in combinations(reduced_buttons.len(), **to_reduce, first) {
        let mut new_pattern = pattern.to_vec().clone();
        // println!("pat: {:?}", new_pattern);
        // println!("combination {:?}", combination);
        // println!("red_btn: {:?}", reduced_buttons);
        for (i, val) in combination.iter().enumerate() {
            let Some(pat) = change_button_2_val(new_pattern, &reduced_buttons[i], val) else {
                continue 'combinations;
            };
            new_pattern = pat;
        }
        // if first && *to_reduce == 50 {
        // println!("Pattern valid");
        // }
        // println!("subtracted pat: {:?}", new_pattern);
        // println!("rest btn: {:?}", new_buttons);
        if let Some(recurse) = recursive2(&new_pattern, &new_buttons, false) {
            // return Some(recurse + *to_reduce);
            if min.is_none_or(|min| min > recurse + *to_reduce) {
                min = Some(recurse + *to_reduce);
            }
        }
    }
    min
    // min.map(|min| to_reduce + min)
}

fn find_all_patterns_2(
    cache: &mut HashMap<Vec<u16>, (u16, Option<u16>)>,
    button_presses: u16,
    pattern: Vec<u16>,
    buttons: Vec<Vec<u16>>,
) -> Option<u16> {
    if pattern.iter().all(|val| *val == 0) {
        println!("res: {button_presses}");
        return Some(button_presses);
    }
    if let Some(val) = cache.get_mut(&pattern) {
        if val.1.is_some_and(|val| button_presses < val) {
            val.1 = Some(val.1.unwrap() - val.0 + button_presses);
            val.0 = button_presses;
        }
        return val.1;
    }

    // let mut found_divisible = (None, (0, None));
    // for (key, val) in cache.iter() {
    //     let found = divisible(key, &pattern);
    //     if found.is_some() {
    //         found_divisible = (found, *val);
    //         break;
    //     }
    // }

    // if let Some(divisor) = found_divisible.0 {
    //     let val = found_divisible.1;
    //     cache.insert(pattern, (val.0 * divisor, val.1.map(|val| val * divisor)));
    //     return val.1.map(|val| val * divisor);
    // }
    if !buttons.is_empty() {
        let mut min: Option<(_, _)> = None;
        for button in buttons.iter() {
            if min.is_some_and(|min| min.1 != button.len()) {
                break;
            }
            let Some(changed_pattern) = change_button_2(pattern.clone(), button) else {
                continue;
            };
            let mut new_buttons = buttons.clone();
            for i in changed_pattern
                .iter()
                .enumerate()
                .filter_map(|(i, val)| if *val == 0 { Some(i) } else { None })
            {
                new_buttons.retain(|button| !button.contains(&(i as u16)));
            }
            if let Some(min_val) =
                find_all_patterns_2(cache, button_presses + 1, changed_pattern, new_buttons)
                && min.is_none_or(|min| min.0 > min_val)
            {
                min = Some((min_val, button.len()));
            }
        }
        cache.insert(pattern, (button_presses, min.map(|min| min.0)));
        return min.map(|min| min.0);
    }
    cache.insert(pattern, (button_presses, None));
    None
}

fn find_shortest_pattern_2(goal: Vec<u16>, mut buttons: Vec<Vec<u16>>) -> u16 {
    let mut patterns = vec![(goal.clone(), 0)];
    buttons.sort_unstable_by_key(|button| button.len());
    loop {
        let mut new_patterns: Vec<(Vec<u16>, u16)> = Vec::new();
        for (pattern, button_presses) in patterns.iter() {
            for button in buttons.iter().rev() {
                let Some(changed_pattern) = change_button_2(pattern.clone(), button) else {
                    continue;
                };
                if let Some(existing_pattern) = new_patterns
                    .iter_mut()
                    .find(|(pattern, _)| *pattern == changed_pattern)
                {
                    existing_pattern.1 = existing_pattern.1.min(button_presses + 1);
                } else {
                    new_patterns.push((changed_pattern.clone(), button_presses + 1));
                    if changed_pattern.iter().all(|val| *val == 0) {
                        println!("{:?}", new_patterns);
                        return button_presses + 1;
                    }
                }
            }
        }

        for (pattern, button_presses) in new_patterns.iter() {
            if let Some(divisor) = divisible(pattern, &goal) {
                return button_presses * divisor;
            }
        }

        patterns = new_patterns;
        patterns.sort_unstable_by_key(|(_, btn_prs)| *btn_prs);
    }
}

fn change_button(mut pattern: Vec<bool>, button: Vec<u16>) -> Vec<bool> {
    for press in button {
        pattern[press as usize] = !pattern[press as usize];
    }
    pattern
}

fn change_button_2(mut pattern: Vec<u16>, button: &Vec<u16>) -> Option<Vec<u16>> {
    for &press in button {
        let new_val = pattern[press as usize].checked_sub(1);
        if let Some(new_val) = new_val {
            pattern[press as usize] = new_val;
        } else {
            return None;
        }
    }
    Some(pattern)
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

fn change_button_2_add(
    mut pattern: Vec<u16>,
    goal: &Vec<u16>,
    button: &Vec<u16>,
) -> Option<Vec<u16>> {
    for &press in button {
        pattern[press as usize] += 1;
        if goal[press as usize] < pattern[press as usize] {
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
        // println!("Working on {i}");
        let start1 = Instant::now();
        let mut all_inputs = line.split_whitespace();
        let _ = all_inputs
            .next()
            .unwrap()
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|char| char == '#')
            .collect::<Vec<_>>();

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
        // println!("{:?}", buttons);
        // let mut cache = HashMap::new();
        let val = recursive2(&joltage, &buttons, true).unwrap();
        // let val = find_all_patterns_2(&mut cache, 0, joltage, buttons).unwrap();
        // let val = find_shortest_pattern_2(joltage, buttons);
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
