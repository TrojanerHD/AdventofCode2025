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
    // let mut start_index = 0;
    for line in input.lines() {
        let (input, outputs) = line.split_once(": ").unwrap();
        connections.insert(input, outputs.split_whitespace().collect::<Vec<_>>());
        // if input == "you" {
        // start_index = i;
        // }
    }

    let mut current = vec!["you"];

    let mut res = 0;
    while let Some(first) = current.pop() {
        let Some(outputs) = connections.get(first) else {
            res += 1;
            continue;
        };
        // res += outputs.len();
        current.extend(outputs);
    }

    res.to_string().to_owned()
}

pub fn part2(input: &str) -> String {
    let mut connections = HashMap::new();
    // let mut start_index = 0;
    for line in input.lines() {
        let (input, outputs) = line.split_once(": ").unwrap();
        connections.insert(input, outputs.split_whitespace().collect::<Vec<_>>());
        // if input == "you" {
        // start_index = i;
        // }
    }
    let mut cache = HashMap::new();
    let dac_to_fft = paths_from_to(&mut cache, &connections, "dac", "fft", "");
    let svr_to_dac = if dac_to_fft != 0 {
        paths_from_to(&mut cache, &connections, "svr", "dac", "fft")
    } else {
        0
    };
    let fft_to_out = if dac_to_fft != 0 {
        paths_from_to(&mut cache, &connections, "fft", "out", "")
    } else {
        0
    };

    let fft_to_dac = paths_from_to(&mut cache, &connections, "fft", "dac", "");

    let svr_to_fft = if fft_to_dac != 0 {
        paths_from_to(&mut cache, &connections, "svr", "fft", "dac")
    } else {
        0
    };
    let dac_to_out = if fft_to_dac != 0 {
        paths_from_to(&mut cache, &connections, "dac", "out", "")
    } else {
        0
    };
    println!(
        "svr_to_dac: {svr_to_dac}, dac_to_fft: {dac_to_fft}, fft_to_out: {fft_to_out}, svr_to_fft: {svr_to_fft}, fft_to_dac: {fft_to_dac}, dac_to_out: {dac_to_out}"
    );

    (svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out)
        .to_string()
        .to_owned()
    // let mut current = vec![("svr", false, false)];

    // let mut res = 0;
    // while let Some(first) = current.pop() {
    //     let Some(outputs) = connections.get(first.0) else {
    //         if first.1 && first.2 {
    //             res += 1;
    //         }
    //         continue;
    //     };
    //     // res += outputs.len();
    //     current.extend(
    //         outputs
    //             .iter()
    //             .map(|output| {
    //                 let dac_visited = first.1 || *output == "dac";
    //                 let ftt_visited = first.2 || *output == "fft";
    //                 (*output, dac_visited, ftt_visited)
    //             })
    //             .collect::<Vec<_>>(),
    //     );
    // }

    // res.to_string().to_owned()
}
