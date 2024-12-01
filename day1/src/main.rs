use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut left_list = vec![];
    let mut right_list = vec![];
    let mut position_found = HashMap::<i32, i32>::new();
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_iter = line.unwrap();
        let mut chunks = line_iter.split_whitespace();
        let left_value = chunks.next().unwrap().parse::<i32>().unwrap();
        let right_value = chunks.next().unwrap().parse::<i32>().unwrap();

        left_list.push(left_value);
        right_list.push(right_value);

        match position_found.get(&right_value) {
            Some(count) => {
                position_found.insert(right_value, count + 1);
            }
            None => {
                position_found.insert(right_value, 1);
            }
        }
    }

    left_list.sort();
    right_list.sort();

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for index in 0..left_list.len() {
        part1_answer += (left_list[index] - right_list[index]).abs();

        match position_found.get(&left_list[index]) {
            Some(count) => {
                part2_answer += left_list[index] * count;
            }
            None => {}
        }
    }

    println!("part 1 answer:{}", part1_answer);
    println!("part 2 answer:{}", part2_answer);
}
