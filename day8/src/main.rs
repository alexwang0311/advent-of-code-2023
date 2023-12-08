use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut instructions: String = String::from("");
        let mut hm: HashMap<String, (String, String)> = HashMap::new();
        let mut nodes: Vec::<String> = Vec::new();

        for line in lines {
            if let Ok(text) = line {
                //println!("{}", text);
                match text.split_once(" = ") {
                    Some((src, dst)) => {
                        //p2
                        if src.chars().last().unwrap() == 'A' {
                            nodes.push(src.to_owned())
;                       }

                        if let Some((left, right)) = dst[1..dst.len()-1].split_once(",") {
                            hm.insert(src.to_owned(), (left.trim().to_owned(), right.trim().to_owned()));
                        }
                        else {
                            panic!("failed to parse line: {}", text);
                        }
                    },
                    None => {
                        if !text.is_empty() {
                            instructions = text;
                        }
                    }
                }
            }
        }
        //println!("{}", instructions);
        //println!("{:?}", hm);
        //println!("{:?}", nodes);

        //p2
        let steps = nodes.iter().map(|src| find_steps(src, &instructions, &hm)).collect::<Vec<usize>>();
        //println!("{:?}", steps);
        let steps = steps.iter().fold(1usize, |m, cur| lcm(m, *cur));
        println!("{:?}", steps);

        //p1
        /* 
        let mut cur: &str = "AAA";
        let mut steps: usize = 0;
        while cur != "ZZZ" {
            let index: usize = steps % instructions.len();
            //println!("index: {}", index);
            match instructions.chars().nth(index).unwrap() {
                'L' => {
                    println!("L: {} -> {}", cur, hm[cur].0);
                    cur = &hm[cur].0;
                }
                _ => {
                    println!("R: {} -> {}", cur, hm[cur].1);
                    cur = &hm[cur].1;
                }
            }
            steps += 1;
        }
        */

        //println!("{}", steps);
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn find_steps(src: &str, instructions: &str, hm: &HashMap<String, (String, String)>) -> usize {
    let mut cur: &str = src;
    let mut steps: usize = 0;
    while cur.chars().last().unwrap() != 'Z' {
        let index: usize = steps % instructions.len();
        //println!("index: {}", index);
        match instructions.chars().nth(index).unwrap() {
            'L' => {
                //println!("L: {} -> {}", cur, hm[cur].0);
                cur = &hm[cur].0;
            }
            _ => {
                //println!("R: {} -> {}", cur, hm[cur].1);
                cur = &hm[cur].1;
            }
        }
        steps += 1;
    }
    steps
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
