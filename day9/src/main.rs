use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut histories: Vec<Vec<i64>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(text) = line {
                //println!("{}", text);
                histories.push(text.split(" ").into_iter().map(|e| e.parse::<i64>().unwrap()).collect::<Vec<i64>>());
            }
        }
    }

    //println!("{:?}", histories);
    let mut vals: Vec<i64> = Vec::new();
    for history in histories {
        //p1
        //vals.push(extrapolate(&history).iter().sum());
        //p2
        vals.push(extrapolate_p2(&history).iter().rev().fold(0, |res, e| e - res));
    }

    println!("{:?}, sum: {}", vals, vals.iter().sum::<i64>());
}

fn extrapolate_p2(history: &Vec<i64>) -> Vec<i64> {
    let mut first_element: Vec<i64> = Vec::new();
    let mut cur = history.clone();
    while !is_all_zero(&cur) {
        //println!("{:?}", cur);
        first_element.push(*cur.first().unwrap());
        cur = cur.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i64>>();
    }
    first_element
}

fn extrapolate(history: &Vec<i64>) -> Vec<i64> {
    let mut last_element: Vec<i64> = Vec::new();
    let mut cur = history.clone();
    while !is_all_zero(&cur) {
        //println!("{:?}", cur);
        last_element.push(*cur.last().unwrap());
        cur = cur.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i64>>();
    }
    last_element
}

//NOTE: this assumes that the vec is sorted, which is ALWAYS the case here
fn is_all_zero(vec: &Vec<i64>) -> bool {
    *(vec.first().unwrap()) == 0 && *(vec.last().unwrap()) == 0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
