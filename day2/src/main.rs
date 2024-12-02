use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file =
        File::open("/Users/etienne_lechat/aoc2024/inputs/day02.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => {
                let diffs: Vec<_> = content
                    .split_whitespace()
                    .map(|e| e.parse::<i16>().unwrap())
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|w| w[0] - w[1])
                    .collect();
                let sign = diffs[0] > 0;
                if diffs
                    .iter()
                    .all(|e| (*e < 0) ^ sign && e.abs() <= 3 && e.abs() >= 1)
                {
                    ans1 += 1;
                    ans2 += 1;
                } else {
                    let nums: Vec<_> = content
                        .split_whitespace()
                        .map(|e| e.parse::<i16>().unwrap())
                        .collect::<Vec<_>>();
                    for idx in 0..nums.len() {
                        let test: Vec<_> = nums
                            .iter()
                            .enumerate()
                            .filter(|&(i, _)| i != idx)
                            .map(|(_, x)| x.clone())
                            .collect();
                        let diffs: Vec<_> = test.windows(2).map(|w| w[0] - w[1]).collect();
                        let sign = diffs[0] > 0;
                        if diffs
                            .iter()
                            .all(|e| (*e < 0) ^ sign && e.abs() <= 3 && e.abs() >= 1)
                        {
                            ans2 += 1;
                            break;
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}
