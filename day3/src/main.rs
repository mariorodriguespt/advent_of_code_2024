use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{0,3})\)").unwrap();
    let enabled = Regex::new(r"do\(\)").unwrap();
    let disabled = Regex::new(r"don't\(\)").unwrap();
    let stdin = io::stdin();
    let mut answer = 0;
    let mut breakpoints = vec![(0,true)];

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        //check
        for item in enabled.captures_iter(&line) {
            let start_index = item.get(0).unwrap().start();
            // println!("{:?}", start_index);
            breakpoints.push((start_index, true));
        }

        for item in disabled.captures_iter(&line) {
            let start_index = item.get(0).unwrap().start();
            // println!("{:?}", start_index);
            breakpoints.push((start_index, false));
        }

        breakpoints.sort_by(|a, b| a.0.cmp(&b.0));

        let mut start_index: i32 = 0;
        let mut end_index : i32= -1;

        while start_index != -1 {
            //find next disabled breakpoint
            for i in start_index as usize..breakpoints.len(){
                if breakpoints[i].1 == false {
                    end_index = i as i32;
                    break;
                }
            }

            let mut chunk;
            if end_index == -1 {
                end_index = (line.len() - 1) as i32;
                chunk = &line[(breakpoints[start_index as usize].0 )..];
            }
            else {
                chunk = &line[(breakpoints[start_index as usize].0 )..(breakpoints[end_index as usize].0 -1)];
            }

            for (_, [f, s]) in re.captures_iter(chunk).map(|c| c.extract()) {
                answer += f.parse::<i32>().unwrap() * s.parse::<i32>().unwrap();
            }

            start_index = -1;
            for i in (end_index+1) as usize..breakpoints.len(){
                if breakpoints[i].1 {
                    start_index = i as i32;
                    break;
                }
            }
        }

        println!("{}", answer);
    }

}
