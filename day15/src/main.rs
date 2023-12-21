use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sequences : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                println!("{}", text);
                for seq in text.split(",") {
                    sequences.push(seq.to_owned());
                }
            }
        }
    }

    println!("{:?}", sequences);

    let hashes = sequences.iter().map(|s| hash(s)).collect::<Vec<usize>>();

    println!("{:?}", hashes);

    println!("sum: {:?}", hashes.iter().sum::<usize>());
}

fn hash(s: &str) -> usize {
    let mut val: usize = 0;
    for c in s.chars() {
        val += c as usize;
        val = (17 * val) % 256;
    }
    val
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
