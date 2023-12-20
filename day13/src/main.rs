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
            /*
            for line in &pattern {
                println!("{}", line);
            }
            println!();
             */

            //total += find_reflection(&pattern);
            total += calculate_score(&pattern);

            //println!();
        }

        println!("{}", total);
    }
}

fn edit_distance(a: &str, b: &str) -> usize {
    let mut d: usize = 0;
    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 != c2 {
            d += 1;
        }
    }
    d
}

fn calculate_score(pattern: &Vec<String>) -> usize {
    //p2
    match find_reflection_p2(pattern) {
        Some(row) => return row * 100,
        None => {
            let pattern = &transpose_pattern(pattern);
            match find_reflection_p2(pattern) {
                Some(col) => return col,
                None => panic!("This should not happen!")
            }
        }
    }
    //p1
    /*
    match find_reflection(pattern) {
        Some(row) => return row * 100,
        None => {
            let pattern = &transpose_pattern(pattern);
            match find_reflection(pattern) {
                Some(col) => return col,
                None => panic!("This should not happen!")
            }
        }
    }
    */
}

fn find_reflection_p2(pattern: &Vec<String>) -> Option<usize> {
    if edit_distance(&pattern[0], &pattern[1]) == 1 {
        return Some(1);
    }
    if edit_distance(&pattern[pattern.len() - 2], &pattern[pattern.len() - 1]) == 1 {
        return Some(pattern.len() - 1);
    }

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
    //println!("rows: {:?}", reflections);
    for (index, v) in reflections {
        if index.min(pattern.len() - index) == (v.len() + 1) {
            let mut indices: Vec<usize> = ((index - v.len() - 1)..(index + v.len() + 1)).collect();
            //println!("row indices: {:?}", indices);
            let mut existing: HashSet<usize> = HashSet::new();
            for pair in v {
                existing.insert(pair.0);
                existing.insert(pair.1);
            }
            indices.retain(|i| !existing.contains(i));
            //println!("remaining: {:?}", indices);
            if edit_distance(&pattern[indices[0]], &pattern[indices[1]]) == 1 {
                //println!("row: {}", index);
                return Some(index);
            }
        }
    }

    None
}

fn find_reflection(pattern: &Vec<String>) -> Option<usize> {
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
    //println!("rows: {:?}", reflections);
    for (index, v) in reflections {
        if index.min(pattern.len() - index) == v.len() {
            //println!("row: {}", index);
            return Some(index);
        }
    }

    None
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

fn transpose_pattern(pattern: &Vec<String>) -> Vec<String> {
    let pattern = pattern.iter().map(|str| str.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let pattern = transpose(pattern).iter().map(|v| v.iter().collect()).collect::<Vec<String>>();

    pattern
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
