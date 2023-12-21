use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                platform.push(text.chars().collect::<Vec<char>>());
            }
        }
    }

    //print_platform(&platform);

    let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let total: usize = 1_000_000_000usize;
    for i in 0..total {
        println!("{}", i);
        platform = spin(&platform);

        if seen.contains_key(&platform) {
            println!("back to {}", seen[&platform]);

            let starting_index = seen[&platform];
            let cycle = i - seen[&platform];
            let remaining = total - i - 1;
            let offset = remaining % cycle;

            let (platform, v) = seen.iter().filter(|&(k, v)| *v == starting_index + offset).next().unwrap();
            println!("cycle: {} | remaining: {} | offset: {} | index: {}", cycle, remaining, offset, v);

            let sum = platform.iter().enumerate().fold(0, |sum, (index, v)| {
                let rocks = v.iter().filter(|&c| *c == 'O').count();
                let multiplier = platform.len() - index;
                let sum = sum + rocks * multiplier;
                sum
            });
        
            println!("{}", sum);

            break;
        }
        else {
            seen.insert(platform.clone(), i);
        }
    }

    //print_platform(&platform);

    //p1
    /* 
    let platform = transpose(platform);

    print_platform(&platform);

    let tilted_platform = tilt(&platform);
    println!("--------------------------------------------");

    let tilted_platform = transpose(tilted_platform);

    print_platform(&tilted_platform);

    let sum = tilted_platform.iter().enumerate().fold(0, |sum, (index, v)| {
        let rocks = v.iter().filter(|&c| *c == 'O').count();
        let multiplier = tilted_platform.len() - index;
        let sum = sum + rocks * multiplier;
        sum
    });

    println!("{}", sum);
    */
}

fn spin(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let platform = transpose(platform.clone());
    let platform = tilt(&platform);
    let platform = transpose(platform); // north
    //println!("-------------north--------------");
    //print_platform(&platform);

    let platform = tilt(&platform); // west
    //println!("-------------west--------------");
    //print_platform(&platform);

    let platform = transpose(platform).iter_mut().map(|v| {
        v.reverse();
        v.clone()
    }).collect::<Vec<Vec<char>>>();
    let mut platform = tilt(&platform);
    let mut platform = transpose(platform.iter_mut().map(|v| {
        v.reverse();
        v.clone()
    }).collect::<Vec<Vec<char>>>()); // south
    //println!("-------------south--------------");
    //print_platform(&platform);

    let platform = platform.iter_mut().map(|v| {
        v.reverse();
        v.clone()
    }).collect::<Vec<Vec<char>>>();
    let mut platform = tilt(&platform);
    let platform = platform.iter_mut().map(|v| {
        v.reverse();
        v.clone()
    }).collect::<Vec<Vec<char>>>(); // east
    //println!("-------------east--------------");
    //print_platform(&platform);

    platform.clone()
}

fn tilt(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_platform: Vec<Vec<char>> = Vec::new();

    for v in platform {
        let col = v.iter().collect::<String>();
        let col = col.replace("#", "|#|");
        let chunks = col.split("|").filter(|c| !c.is_empty());
        
        let mut tilted = String::from("");
        for chunk in chunks {
            if chunk.chars().filter(|c| *c == '#').count() > 0 {
                tilted.push_str("#");
            }
            else {
                let rocks: usize = chunk.chars().filter(|&c| c == 'O').count();
                let empty_space: usize = chunk.len() - rocks;
                let new_area = vec!['O'; rocks].iter().chain(vec!['.'; empty_space].iter()).collect::<String>();
                tilted.push_str(&new_area);
            }
            //print!("{}|", chunk);
        }
        //println!("tilted: {}", tilted);
        tilted_platform.push(tilted.chars().collect::<Vec<char>>());
    }
    //println!("--------------------------------------------");

    tilted_platform
}

fn print_platform(platform: &Vec<Vec<char>>) {
    for v in platform {
        println!("{}", v.iter().collect::<String>());
    }
    //println!("--------------------------------------------");
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
