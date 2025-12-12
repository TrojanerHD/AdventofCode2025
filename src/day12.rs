pub fn part1(input: &str) -> String {
    input
        .lines()
        .skip_while(|line| !line.contains("x"))
        .filter(|line| {
            let (size, indices) = line.split_once(": ").unwrap();
            let (x_size, y_size) = size.split_once("x").unwrap();
            x_size.parse::<u16>().unwrap() * y_size.parse::<u16>().unwrap()
                >= 9_u16
                    * indices
                        .split_whitespace()
                        .map(|index| index.parse::<u16>().unwrap())
                        .sum::<u16>()
        })
        .count()
        .to_string()
        .to_owned()
}

pub fn part2(_input: &str) -> String {
    "".to_owned()
}
