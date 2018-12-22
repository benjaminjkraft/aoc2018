use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

fn counter<T: Eq + Hash, I: Iterator<Item = T>>(items: I) -> HashMap<T, u32> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

fn part1<'a, I: Iterator<Item = &'a String>>(lines: I) -> i32 {
    let mut counts = HashMap::new();
    for line in lines {
        let char_counts = counter(line.chars());
        let vals: HashSet<&u32> = char_counts.values().collect();
        for val in vals {
            *counts.entry(val.clone()).or_insert(0) += 1;
        }
    }
    counts.get(&2).unwrap_or(&0) * counts.get(&3).unwrap_or(&0) 
}

fn part2<'a, I: Iterator<Item = &'a String> + Clone>(lines: I) -> String {
    for line1 in lines.clone() {
        for line2 in lines.clone() {
            let mut diffs = 0;
            let mut line = String::new();
            for (c1, c2) in line1.chars().zip(line2.chars()) {
                if c1 != c2 {
                    diffs += 1;
                    if diffs > 1 {
                        break
                    }
                } else {
                    line.push(c1);
                }
            }
            if diffs == 1 {
                return line;
            }
        }
    }
    String::new()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .collect::<Result<_, _>>()
        .expect("expected input");
    println!("{}", part1(lines.iter()));
    println!("{}", part2(lines.iter()));
}
