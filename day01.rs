use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn part1<'a, I: Iterator<Item = &'a i32>>(nums: I) -> i32 {
    nums.sum()
}

fn part2<'a, I: Iterator<Item = &'a i32> + Clone>(nums: I) -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    for num in nums.cycle() {
        sum += num;
        if seen.contains(&sum) {
            break;
        }
        seen.insert(sum);
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .collect::<Result<_, _>>()
        .expect("expected input");
    let nums: Vec<i32> = lines
        .iter()
        .map(|x| x.parse().expect("expected an int"))
        .collect();
    println!("{}", part1(nums.iter()));
    println!("{}", part2(nums.iter()));
}
