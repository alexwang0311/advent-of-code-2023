use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total: usize = 0;
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                if let Some((pattern, groups)) = text.split_once(" ") {
                    //p2
                    let pattern = (pattern.to_owned() + "?").repeat(5);
                    let pattern = &pattern[..pattern.len() - 1];
                    let groups = (groups.to_owned() + ",").repeat(5);
                    let groups = &groups[..groups.len() - 1];

                    let groups = groups.split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
                    total += count_all_combinations(&groups, pattern, &mut cache);
                }
            }
        }

        println!("{}", total);
    }
}

fn count_all_combinations(groups: &Vec<usize>, pattern: &str, cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    if pattern.is_empty() {
        match groups.len() {
            0 => return 1,
            _ => return 0
        }
    }

    if groups.len() == 0 {
        match pattern.chars().filter(|c| *c == '#').count() {
            0 => return 1,
            _ => return 0
        }
    }

    let key = (pattern.to_owned(), groups.clone());
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut count: usize = 0;
    let first = pattern.chars().next().unwrap();
    if first == '.' || first == '?' {
        count += count_all_combinations(groups, &pattern[1..], cache);
    }
    if first == '#' || first == '?' {
        if groups[0] <= pattern.len() && pattern[..groups[0]].chars().filter(|c| *c == '.').count() == 0 && (groups[0] == pattern.len() || pattern.chars().nth(groups[0]).unwrap() != '#') {
            if groups[0] + 1 >= pattern.len() {
                count += count_all_combinations(&groups[1..].to_vec(), "", cache);
            }
            else {
                count += count_all_combinations(&groups[1..].to_vec(), &pattern[(groups[0] + 1)..], cache);
            }
        }
    }

    cache.insert(key, count);

    return count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}