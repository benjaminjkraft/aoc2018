use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

fn count_into<T: Eq + Hash, I: IntoIterator<Item = T>>(counts: &mut HashMap<T, i32>, items: I) {
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
}

fn part1<'a, I: Iterator<Item = &'a String>>(lines: I) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let mut char_counts = HashMap::new();
        count_into(&mut char_counts, line.chars());
        let mut char_set: HashSet<i32> = char_counts.values().map(|v| v.clone()).collect();
        count_into(&mut counts, char_set);
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
