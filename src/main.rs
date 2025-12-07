use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    time::Instant,
};

use anyhow::{Context, Result, bail};

include!(concat!(env!("OUT_DIR"), "/runner.rs"));

fn main() -> Result<()> {
    let day = if let Some(arg) = std::env::args().nth(1) {
        arg.trim().parse()?
    } else {
        let day = runner::current_day();
        if day == 0 {
            bail!("no solution files found");
        }
        day
    };

    if !(1..=12).contains(&day) {
        bail!("day {day} is not a valid advent of code day");
    }
    let start = Instant::now();
    let input = setup(day)?;
    println!("Read input file: {:?}", start.elapsed());

    println!();

    println!("Running day {day} part 1:");
    let start = Instant::now();
    let result = runner::run_part1(day, &input);
    println!("took {:?}", start.elapsed());
    println!("output: {result}");

    println!();

    println!("Running day {day} part 2:");
    let start = Instant::now();
    let result = runner::run_part2(day, &input);
    println!("took {:?}", start.elapsed());
    println!("output: {result}");
    Ok(())
}

fn setup(day: u32) -> Result<String> {
    let input_dir = Path::new("input");
    fs::create_dir_all(input_dir)?;
    let input_file = input_dir.join(format!("day{day}.txt"));
    let mut input = String::new();
    match File::open(&input_file) {
        Ok(mut f) => {
            f.read_to_string(&mut input)?;
        }
        Err(_) => {
            let token = fs::read_to_string(".token.txt")
                .context("session token file '.token.txt' not found")?;
            input = ureq::get(&format!("https://adventofcode.com/2025/day/{day}/input"))
                .set("Cookie", &format!("session={}", token.trim()))
                .call()?
                .into_string()?;
            fs::write(input_file, &input)?;
        }
    };
    Ok(input)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::runner;
    use anyhow::{Result, bail};

    #[test]
    fn current_day() -> Result<()> {
        let day = runner::current_day();
        if day == 0 {
            bail!("no solution files found");
        }

        test_day(day)
    }

    fn test_day(day: u32) -> Result<()> {
        let test_dir = Path::new("test");
        fs::create_dir_all(test_dir)?;
        let mut empty = true;
        for input_file in fs::read_dir(test_dir).unwrap().filter(|file| {
            let file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
            file_name.starts_with(format!("test{day}.").as_str())
                || file_name.starts_with(format!("test{day}_").as_str())
        }) {
            empty = false;
            let file_content = fs::read_to_string(input_file.as_ref().unwrap().path())?;
            let mut lines_back = file_content.lines().rev();
            let output2 = lines_back.next().unwrap();
            let output1 = lines_back.next().unwrap();
            let input = lines_back
                .rev()
                .map(|line| line.to_owned())
                .reduce(|acc, line| format!("{acc}\n{line}"))
                .unwrap();

            let part1_res = runner::run_part1(day, input.as_str());
            let file_name = input_file.unwrap().file_name().into_string().unwrap();
            assert_eq!(
                part1_res, output1,
                "day {day} part 1: {} failed. Expected output: {output1}, actual output: {part1_res}",
                file_name
            );
            let part2_res = runner::run_part2(day, input.as_str());
            assert_eq!(
                part2_res, output2,
                "day {day} part 2: {} failed. Expected output: {output2}, actual output: {part2_res}",
                file_name
            );
        }
        if empty {
            bail!("No test inputs for day {day}");
        } else {
            println!("Day {day} tested");
        }
        Ok(())
    }

    #[test]
    #[ignore]
    fn all_days() -> Result<()> {
        for day in 1..=runner::current_day() {
            test_day(day)?
        }
        Ok(())
    }
}
