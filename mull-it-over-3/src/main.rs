const INPUT: &str = include_str!("input.txt");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let sum: i32 = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap().captures_iter(INPUT)
        .map(|capture| capture.extract())
        .map(|(_, [a, b])| {
            let a = a.parse::<i32>().expect("not a number");
            let b = b.parse::<i32>().expect("not a number");
            a * b
        }).sum();

    println!("Part One: {}", sum);
}

fn part_two() {
    let mut sum = true;
    let sum: i32 = regex::Regex::new(r"do\(\)()()|don't\(\)()()|mul\(([0-9]+),([0-9]+)\)").unwrap().captures_iter(INPUT)
        .map(|capture| {
            let (value, [a, b]) = capture.extract();
            if value.starts_with("don't") {
                sum = false;
                return 0;
            } else if value.starts_with("do") {
                sum = true;
                return 0;
            }

            if !sum {
                return 0;
            }

            let a = a.parse::<i32>().expect("not a number");
            let b = b.parse::<i32>().expect("not a number");
            a * b
        }).sum();

    println!("Part Two: {}", sum);
}