const INPUT: &str = include_str!("input.txt");

fn main() {
    part_one();
    part_two();
}

fn is_bad_difference(difference: i32) -> bool {
    difference == 0 || difference.abs() > 3
}

fn are_bad_differences(differences: impl Iterator<Item = i32> + Clone) -> bool {
    if !(differences.clone().all(|diff| diff > 0) || differences.clone().all(|diff| diff < 0)) {
        return true;
    }

    if differences.clone().any(is_bad_difference) {
        return true;
    }

    false
}

fn test_report(report: impl Iterator<Item = i32> + Clone) -> bool {
    let differences = report.clone()
        .zip(report.skip(1))
        .map(|(a, b)| b - a);

    !are_bad_differences(differences)
}

fn part_one() {
    let safe_reports = INPUT.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("not a number"))
        }).filter(|report| test_report(report.clone()))
        .count();

    println!("Part One: {}", safe_reports);
}

fn part_two() {
    let safe_reports = INPUT.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("not a number"))
        }).filter(|report| {
            for (i, _) in report.clone().enumerate() {
                let mut report = report.clone().collect::<Vec<_>>();
                report.remove(i);

                if test_report(report.into_iter()) {
                    return true;
                }
            }

            false
        }).count();

    println!("Part Two: {}", safe_reports);
}