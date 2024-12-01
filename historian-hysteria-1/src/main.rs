use std::{collections::HashMap, vec};

const INPUT: &str = include_str!("input.txt");

fn main() {
    part_one();
    part_two();
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let mut a = vec![];
    let mut b = vec![];

    for line in INPUT.lines() {
        let mut parts = line.split_whitespace();
        a.push(parts.next().expect("no first part").parse::<i32>().expect("not a number"));
        b.push(parts.next().expect("no second part").parse::<i32>().expect("not a number"));
    }

    (a, b)
}

fn part_one() {
    let (mut a, mut b) = get_lists();

    a.sort();
    b.sort();

    let sum: u32 = a.iter().zip(b.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    println!("Part One: {}", sum);
}

fn part_two() {
    let (a, b) = get_lists();

    let mut map = HashMap::new();
    for i in b {
        map.entry(i).and_modify(|i| *i += 1).or_insert(1);
    }

    let sum: i32 = a.iter().map(|i| i * *map.entry(*i).or_default()).sum();
    println!("Part Two: {}", sum);
}