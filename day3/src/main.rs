use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut schema: Vec<Vec<char>> = Vec::new();
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(text) = line {
                schema.push(text.chars().collect::<Vec<char>>());
            }
        }
    }

    for row in 0..schema.len() {
        //println!("Element at position {}: {:?}", row, line);
        for col in 0..schema[0].len() {
            let c: char = schema[row][col];
            if !c.is_alphanumeric() && c != '.' {
                let x: i32 = row.try_into().unwrap();
                let y: i32 = col.try_into().unwrap();
                println!("special character at {},{}", x, y);
                search(x - 1, y - 1, &mut schema, &mut sum);
                search(x - 1, y, &mut schema, &mut sum);
                search(x - 1, y + 1, &mut schema, &mut sum);
                search(x, y - 1, &mut schema, &mut sum);
                search(x, y + 1, &mut schema, &mut sum);
                search(x + 1, y - 1, &mut schema, &mut sum);
                search(x + 1, y, &mut schema, &mut sum);
                search(x + 1, y + 1, &mut schema, &mut sum);
            }
        } 
    }
    println!("{}", sum);
}

fn find_number(x: i32, y:i32, schema: &mut Vec<Vec<char>>, sum: &mut i32) {
    let mut number: String = schema[x as usize][y as usize].to_string();
    //println!("number found at {},{}: {}", x, y, number);
    //search left
    let mut left: i32 = y - 1;
    while left >= 0 && schema[x as usize][left as usize].is_digit(10) {
        number = schema[x as usize][left as usize].to_string() + &number;
        schema[x as usize][left as usize] = '.';
        left -= 1;
    }
    let mut right: i32 = y + 1;
    while right < schema[0].len().try_into().unwrap() && schema[x as usize][right as usize].is_digit(10) {
        number = number + &schema[x as usize][right as usize].to_string();
        schema[x as usize][right as usize] = '.';
        right += 1;
    }
    //println!("{}", number);
    *sum += number.parse::<i32>().unwrap();
}

fn search(x: i32, y: i32, schema: &mut Vec<Vec<char>>, sum: &mut i32) {
    let x_max: usize = schema.len();
    let y_max: usize = schema[0].len();
    if x >= 0 && y >= 0 && x < x_max.try_into().unwrap() && y < y_max.try_into().unwrap() {
        let c: char = schema[x as usize][y as usize];
        if c.is_digit(10) {
            find_number(x, y, schema, sum);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
