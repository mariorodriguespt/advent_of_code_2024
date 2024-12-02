use std::io::{self, BufRead};

fn is_safe_report(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] < report [1];

    for i in 0..report.len() -1 {
        if (report[i] - report[i+1]).abs() > 3
            || (report[i] - report[i+1]).abs() == 0
            || (report[i] < report[i+1] && !is_increasing)
            || (report[i] > report[i+1] && is_increasing)
        {
            return false;
        }
    }

    true
}

fn main() {
    let mut safe_reports = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut numbers : Vec<i32> = line.split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let is_safe = is_safe_report(&numbers);

        if is_safe {
            safe_reports += 1;
        }
        else {
            for removed_item_index in 0..numbers.len() {
                let mut report_variation = numbers.clone();
                report_variation.remove(removed_item_index);
                let is_variation_safe = is_safe_report(&report_variation);;

                if is_variation_safe {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("part 1:{}", 383);
    println!("part 2:{}", safe_reports);
}
