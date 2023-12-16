use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut patterns: Vec<Vec<String>> = Vec::new();
        let mut pattern: Vec<String> = Vec::new();
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                if text.is_empty() {
                    patterns.push(pattern.clone());
                    pattern.clear();
                }
                else {
                    pattern.push(text);
                }
            }
        }
        patterns.push(pattern);

        let mut total: usize = 0;
        for pattern in patterns {
            for line in &pattern {
                println!("{}", line);
            }
            println!();

            total += find_reflection(&pattern);

            println!();
        }

        println!("{}", total);
    }
}

fn find_reflection(pattern: &Vec<String>) -> usize {
    let row_groups = map_rows(pattern).values().filter(|&v| v.len() >= 2).cloned().collect::<Vec<Vec<usize>>>();
    let mut reflections: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for group in row_groups {
        let pairs = transform_into_pairs(&group);
        for pair in pairs {
            if (pair.0 + pair.1) % 2 == 1 {
                let count = reflections.entry((pair.0 + pair.1) / 2 + 1).or_insert(Vec::new());
                count.push(pair);
            }
        }
    }
    println!("rows: {:?}", reflections);
    for kv in reflections {
        if kv.0.min(pattern.len() - kv.0) == kv.1.len() {
            println!("row: {}", kv.0);
            return kv.0 * 100;
        }
    }

    let col_groups = map_cols(pattern).values().filter(|&v| v.len() >= 2).cloned().collect::<Vec<Vec<usize>>>();
    let mut reflections: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for group in col_groups {
        let pairs = transform_into_pairs(&group);
        for pair in pairs {
            if (pair.0 + pair.1) % 2 == 1 {
                let count = reflections.entry((pair.0 + pair.1) / 2 + 1).or_insert(Vec::new());
                count.push(pair);
            }
        }
    }
    println!("cols: {:?}", reflections);
    for kv in reflections {
        if kv.0.min(pattern[0].len() - kv.0) == kv.1.len() {
            println!("col: {}", kv.0);
            return kv.0;
        }
    }

    panic!("this should not happen")
}

fn transform_into_pairs(groups: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..groups.len() {
        for j in (i + 1)..groups.len() {
            pairs.push((groups[i], groups[j]));
        }
    }
    pairs
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

fn map_cols(pattern: &Vec<String>) -> HashMap<String, Vec<usize>> {
    let pattern = pattern.iter().map(|str| str.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let pattern = transpose(pattern).iter().map(|v| v.iter().collect()).collect::<Vec<String>>();

    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for (col, line) in pattern.iter().enumerate() {
        let indices = map.entry(line.to_string()).or_insert(Vec::new());
        indices.push(col);
    }

    //println!("{:?}", map);
    map
}

fn map_rows(pattern: &Vec<String>) -> HashMap<String, Vec<usize>> {
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for (row, line) in pattern.iter().enumerate() {
        let indices = map.entry(line.to_string()).or_insert(Vec::new());
        indices.push(row);
    }
    map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
