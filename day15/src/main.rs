use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Box {
    lens: Vec<(String, usize)>
}

impl Box {
    fn add(&mut self, label: &str, focal_len: usize) {
        for pair in self.lens.iter_mut() {
            if pair.0 == label {
                pair.1 = focal_len;
                return;
            }
        }
        self.lens.push((label.to_owned(), focal_len));
    }

    fn remove(&mut self, label: &str) {
        self.lens = self.lens.iter().filter(|(l, _)| l != &label).map(|t| t.clone()).collect();
    }

    fn is_empty(&self) -> bool {
        self.lens.len() == 0
    }

    fn score(&self) -> usize {
        self.lens.iter().enumerate().fold(0, |sum, (index, (_, focal_len))| sum + (index + 1) * focal_len)
    }
}

fn main() {
    let mut sequences : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                for seq in text.split(",") {
                    sequences.push(seq.to_owned());
                }
            }
        }
    }

    //println!("{:?}", sequences);

    let mut boxes: HashMap<usize, Box> = HashMap::new();
    for seq in sequences {
        if seq.contains("=") {
            let (label, focal_len) = seq.split_once("=").unwrap();
            let focal_len = focal_len.parse::<usize>().unwrap();
            let box_num = hash(label);
            //println!("= | hash: {} | label: {} | focal length: {}", box_num, label, focal_len);
            let b = boxes.entry(box_num).or_insert(Box::default());
            b.add(label, focal_len);
        }
        else {
            let label = seq.split("-").next().unwrap();
            let box_num = hash(label);
            //println!("- | hash: {} | label: {}", box_num, label);
            if boxes.contains_key(&box_num) {
                boxes.get_mut(&box_num).unwrap().remove(&label);
                if boxes[&box_num].is_empty() {
                    boxes.remove(&box_num);
                }
            }
        }
    }

    //println!("{:?}", boxes);
    //p2
    println!("{}", boxes.iter().fold(0, |sum, (k, v)| sum + (k + 1) * v.score()));
    //p1
    /* 
    let hashes = sequences.iter().map(|s| hash(s)).collect::<Vec<usize>>();

    println!("{:?}", hashes);

    println!("sum: {:?}", hashes.iter().sum::<usize>());
    */
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
