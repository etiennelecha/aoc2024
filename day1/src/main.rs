use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file =
        File::open("/Users/etienne_lechat/aoc2024/inputs/day01.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let (mut l1, mut l2): (Vec<u64>, Vec<u64>) = reader
        .lines()
        .map(|line| {
            let res = line
                .unwrap()
                .split_whitespace()
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (res[0], res[1])
        })
        .unzip();
    l1.sort();
    l2.sort();
    let ans1: u64 = l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| (a).abs_diff(*b))
        .sum();
    println!("{}", ans1);
    let count_map: HashMap<_, usize> = l2.iter().fold(HashMap::new(), |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    let ans2: u64 = l1
        .iter()
        .map(|e| *e * *count_map.get(e).unwrap_or(&0) as u64)
        .sum();
    println!("{}", ans2);
}
