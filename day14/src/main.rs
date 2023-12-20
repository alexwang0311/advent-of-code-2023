use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./test.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                platform.push(text.chars().collect::<Vec<char>>());
            }
        }
    }

    print_platform(&platform);

    let platform = transpose(platform);

    print_platform(&platform);

    let mut tilted_platform: Vec<String> = Vec::new();

    for v in platform {
        let col = v.iter().collect::<String>();
        let chunks = col.split("#");
        
        let mut tilted = String::from("");
        for chunk in chunks {
            if chunk.is_empty() {
                tilted.push_str("#");
            }
            else {
                let rocks: usize = chunk.chars().filter(|&c| c == 'O').count();
                let empty_space: usize = chunk.len() - rocks;
                let new_area = vec!['O'; rocks].iter().chain(vec!['.'; empty_space].iter()).collect::<String>();
                tilted.push_str(&new_area);
            }
            print!("{}|", chunk);
        }
        println!("tilted: {}", tilted);
        tilted_platform.push(tilted);
    }
}

fn print_platform(platform: &Vec<Vec<char>>) {
    for v in platform {
        println!("{}", v.iter().collect::<String>());
    }
    println!("--------------------------------------------");
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
