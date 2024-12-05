use std::io::{self, BufRead};


fn next_char(current_char: char) -> char {
    match current_char {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => '.'
    }
}

const directions: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1,-1),
    (-1,1),
];

fn dfs(map : &Vec<String>, w: i32, h : i32, x: i32, y: i32, target_char: char, direction: usize) -> bool {
    //bound checking
    if x<0 || y<0 || x>w || y>h {
        return false;
    }
    if map[y as usize].chars().nth(x as usize).unwrap() == target_char {

        if target_char == 'S' {
            return true;
        }
        let next_char = next_char(target_char);
        return dfs(map, w, h, x+directions[direction].0, y+directions[direction].1, next_char, direction);
    }

    false
}

fn mas_exists(map : &Vec<String>, w: i32, h : i32, x: i32, y: i32) -> bool {
    if x<1 || y<1 || x>w-1 || y>h-1 {
        return false;
    }

    if map[y as usize].chars().nth(x as usize).unwrap() != 'A' {
        return false
    }

    let word1 = format!("{}A{}", map[(y-1) as usize].chars().nth((x-1) as usize).unwrap(), map[(y+1) as usize].chars().nth((x+1) as usize).unwrap());
    let word2 = format!("{}A{}", map[(y-1) as usize].chars().nth((x+1) as usize).unwrap(), map[(y+1) as usize].chars().nth((x-1) as usize).unwrap());

    if (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM") {
        return true
    }



    false
}

fn main() {
    let mut map = Vec::<String>::new();
    let mut width = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        width = line.len() as i32;
        map.push(line);
    }

    let mut height=map.len() as i32;

    let mut answer = 0;
    let mut part2_answer =0;
    for x in 0..width {
        for y in 0..height {
            for direction in 0..directions.len(){
                if dfs(&map, width -1, height -1, x as i32, y as i32,  'X', direction) {
                    answer +=1;
                }
            }

            //part 2
            if mas_exists(&map, width-1, height-1, x as i32, y as i32) {
                part2_answer+=1;
            }

        }
    }

    println!("part 1 {}", answer);
    println!("part 2 {}", part2_answer);
}
