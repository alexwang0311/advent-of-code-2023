use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: u32 = 0;
        let mut card_to_match_num: HashMap<u32, u32> = HashMap::new();
        let mut card_nums: HashMap<u32, u32> = HashMap::new();
        let mut count: u32 = 1;
        for line in lines {
            if let Ok(text) = line {
                if let Some((win_nums, card)) = text.split_once(" | ") {
                    let mut found: u32 = 0;
                    let win_nums = win_nums.split(": ").collect::<Vec<&str>>()[1];
                    let mut win_nums = win_nums.split(" ").collect::<Vec<&str>>().iter()
                                                                                    .filter(|num| !num.is_empty())
                                                                                    .map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    let mut card = card.split(" ").collect::<Vec<&str>>().iter()
                                                        .filter(|num| !num.is_empty())
                                                        .map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    card.sort();
                    //println!("{:?} | {:?}", win_nums, card);
                    for win_num in win_nums {
                        //NOTE: should do a binary search here but not sure what the Rust API is
                        if let Some(index) = card.iter().position(|&num| num == win_num) {
                            //println!("found {} at {}", win_num, index);
                            found += 1;
                        }
                    }
                    //p2
                    card_to_match_num.insert(count, found);
                    //p1
                    /*
                    if found > 0 {
                        let points: i32 = i32::pow(2, found - 1);
                        println!("points: {}", points);
                        sum += points;
                    }
                    */
                }
            }
            card_nums.insert(count, 1);
            count += 1;
        }
        //println!("{:?}, {:?}", card_to_match_num, card_nums);
        //println!("{}", sum);
        for i in 1..count {
            let matches = card_to_match_num[&i];
            for j in 1..(matches + 1) {
                let index: u32 = i + j;
                //println!("{}", index);
                card_nums.insert(index, card_nums[&index] + card_nums[&i]);
            }
            //println!("{:?}", card_nums);
        }
        //println!("{:?}", card_nums);
        for i in 1..count {
            sum += card_nums[&i];
        }
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
