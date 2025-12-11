use std::collections::HashMap;

fn paths_from_to(
    cache: &mut HashMap<(String, String), u64>,
    connections: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
    not: &str,
) -> u64 {
    if from == not {
        return 0;
    }
    let Some(outputs) = connections.get(from) else {
        return 0;
    };
    if let Some(res) = cache.get(&(from.to_owned(), to.to_owned())) {
        return *res;
    }
    let mut res = 0;
    for output in outputs {
        if *output == to {
            res += 1;
            continue;
        }
        res += paths_from_to(cache, connections, output, to, not);
    }

    cache.insert((from.to_owned(), to.to_owned()), res);

    // println!("paths from {from} to {to}: {res}");

    res
}

pub fn part1(input: &str) -> String {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (input, outputs) = line.split_once(": ").unwrap();
        connections.insert(input, outputs.split_whitespace().collect::<Vec<_>>());
    }

    let mut current = vec!["you"];

    let mut res = 0;
    while let Some(first) = current.pop() {
        let Some(outputs) = connections.get(first) else {
            res += 1;
            continue;
        };
        current.extend(outputs);
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (input, outputs) = line.split_once(": ").unwrap();
        connections.insert(input, outputs.split_whitespace().collect::<Vec<_>>());
    }

    let mut cache = HashMap::new();
    let dac_to_fft = paths_from_to(&mut cache, &connections, "dac", "fft", "");
    if dac_to_fft != 0 {
        let svr_to_dac = paths_from_to(&mut cache, &connections, "svr", "dac", "fft");
        let fft_to_out = paths_from_to(&mut cache, &connections, "fft", "out", "");

        return (svr_to_dac * dac_to_fft * fft_to_out)
            .to_string()
            .to_owned();
    }

    let fft_to_dac = paths_from_to(&mut cache, &connections, "fft", "dac", "");

    if fft_to_dac == 0 {
        return "No connections from dac to fft or vice versa".to_owned();
    }

    let svr_to_fft = paths_from_to(&mut cache, &connections, "svr", "fft", "dac");
    let dac_to_out = paths_from_to(&mut cache, &connections, "dac", "out", "");

    (svr_to_fft * fft_to_dac * dac_to_out)
        .to_string()
        .to_owned()
}
