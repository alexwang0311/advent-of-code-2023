use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

fn main() {
    let mut sum: u32 = 0;
    let nums: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(text) = line {
                let digits: Vec<char> = text.chars().filter(|c| c.is_digit(10)).collect();
                if digits.len() > 0{
                    let firstDigit = digits[0];
                    let lastDigit = digits[digits.len() - 1];
                    //sum += firstDigit.to_digit(10).unwrap() * 10 + lastDigit.to_digit(10).unwrap();
                    let firstDigitIndex = text.find(firstDigit);
                    let lastDigitIndex = text.rfind(lastDigit);
                    //println!("{} {}", firstDigitIndex.unwrap(), lastDigitIndex.unwrap());
                    let beforeFirstDigit = text.substring(0, firstDigitIndex.unwrap());
                    let afterLastDigit = text.substring(lastDigitIndex.unwrap() + 1, text.len());
                    //println!("{} {}", beforeFirstDigit, afterLastDigit);
                    let firstOccurenceIndices: Vec<Option<usize>> = nums.iter().map(|num| beforeFirstDigit.find(num)).collect();
                    let lastOccurenceIndices: Vec<Option<usize>> = nums.iter().map(|num| afterLastDigit.rfind(num)).collect();
                    //println!("{:?} {:?}", firstOccurenceIndices, lastOccurenceIndices);
                    let mut smallest = usize::MAX;
                    let mut startNum = firstDigit.to_digit(10).unwrap();
                    for (index, e) in firstOccurenceIndices.iter().enumerate() {
                        if let Some(pos) = e {
                            if pos < &smallest {
                                startNum = (index + 1) as u32;
                                smallest = *pos;
                            }
                        }
                    }

                    let mut largest = usize::MIN;
                    let mut endNum = lastDigit.to_digit(10).unwrap();
                    for (index, e) in lastOccurenceIndices.iter().enumerate() {
                        if let Some(pos) = e {
                            if pos >= &largest {
                                endNum = (index + 1) as u32;
                                largest = *pos;
                            }
                        }
                    }

                    //println!("{}", text);
                    //println!("{:?} {:?}", firstOccurenceIndices, lastOccurenceIndices);
                    //println!("{}{}", startNum, endNum);
                    sum += startNum * 10 + endNum;
                }
                else {
                    println!("{}", text);
                }
            }
        }
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}