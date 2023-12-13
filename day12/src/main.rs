use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_arrangements: usize = 0;
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                if let Some((record, groups)) = text.split_once(" ") {
                    //p2
                    /*
                    let record = (record.to_owned() + "?").repeat(5);
                    let record = &record[..record.len() - 1];
                    let groups = (groups.to_owned() + ",").repeat(5);
                    let groups = &groups[..groups.len() - 1];
                    */
                    let arrangements: usize = find_arrangements(record, &groups.split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>());
                    //println!("{}", arrangements);
                    total_arrangements += arrangements;
                    println!("{}: {} | {} | total: {}", row, record, groups, total_arrangements);
                }
            }
        }

        println!("{}", total_arrangements);
    }
}

fn generate_spring_configurations(spring_len: usize, total_len: usize) -> Vec<String> {
    //println!("{} | {}", spring_len, total_len);
    let mut configurations: Vec<String> = Vec::new();
    for i in 0..(total_len - spring_len + 1) {
        let configuration = [vec!['.'; i].as_slice(), vec!['#'; spring_len].as_slice()].concat();
        for j in 0..(total_len - i - spring_len + 1) {
            let configuration = [configuration.as_slice(), vec!['.'; j].as_slice()].concat();
            let configuration = configuration.into_iter().collect::<String>();
            configurations.push(configuration);
        }
    }
    configurations
}

//P2 idea: reduce recursion # by forwarding the current sequences into recursive calls
//consider a pattern p = p1, p2, ..., pn; the total comb is comb(p1) * comb(p2) * ... * comb(pn)
fn generate_all_combinations(groups: &Vec<usize>, pattern: &str, prev: String, seen: &mut HashMap<String, HashSet<String>>) -> HashSet<String> {
    //println!("groups: {:?} | prev: {}", groups, prev);
    if seen.contains_key(&prev) {
        //println!("seen: {} | {:?}", prev, seen[&prev]);
        return seen[&prev].clone();
    }

    let mut combos: HashSet<String> = HashSet::new();

    if groups.len() == 0 {
        if pattern[prev.len()..].chars().into_iter().filter(|c| *c == '#').count() == 0 {
            let end = vec!['.'; pattern.len() - prev.len()].into_iter().collect::<String>();
            combos.insert(format!("{prev}{end}"));
        }
        seen.insert(prev.to_owned(), combos.clone());
        return combos;
    }

    if prev.len() + groups[1..].into_iter().sum::<usize>() + groups[1..].len() >= pattern.len() {
        seen.insert(prev.to_owned(), combos.clone());
        return combos;
    }

    let first_spring_len = groups[0];
    let first_section_len = pattern.len() - prev.len() - groups[1..].into_iter().sum::<usize>() - groups[1..].len();
    let first_spring_configurations = generate_spring_configurations(first_spring_len, first_section_len);
    let first_spring_configurations = first_spring_configurations.into_iter().filter(|configuration| match_pattern(configuration, &pattern[prev.len()..])).collect::<Vec<String>>();
    let first_spring_configurations = first_spring_configurations.into_iter().filter(|configuration| {
        match prev.chars().last() {
            Some(c) => {
                match c {
                    '.' => true,
                    _ => configuration.chars().next().unwrap() != '#'
                }
            },
            None => true
        }
    }).map(|configuration| format!("{prev}{configuration}")).collect::<Vec<String>>();
    //println!("first: {:?}", first_spring_configurations);

    for first_configuration in first_spring_configurations {
        let rest_configurations = generate_all_combinations(&groups[1..].to_vec(), pattern, first_configuration.clone(), seen);
        //println!("rest: {} | {:?}", prev, rest_configurations);
        seen.insert(first_configuration, rest_configurations.clone());

        for rest_configuration in rest_configurations {
            combos.insert(rest_configuration);
        }
    }

    combos
}

fn match_pattern(str: &str, pattern: &str) -> bool {
    for (c, p) in str.chars().zip(pattern.chars()) {
        if c != p && p != '?' {
            return false
        }
    }
    true
}

fn find_arrangements(record: &str, groups: &Vec<usize>) -> usize {
    //println!("{} | {:?}", record, groups);
    let mut seen: HashMap<String, HashSet<String>> = HashMap::new();
    let combos = generate_all_combinations(groups, record, String::from(""), &mut seen);
    //println!("all possible combinations: {:?}", combos);
    combos.len()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_generate_spring_configurations() {
    let res = generate_spring_configurations(3, 12);
    let pattern = "?###????????";
    let res = res.into_iter().filter(|configuration| match_pattern(configuration, pattern)).collect::<Vec<String>>();
    println!("{:?}", res);
}

#[test]
fn test_generate_all_combinations() {
    let mut seen: HashMap<String, HashSet<String>> = HashMap::new();
    //let pattern = "?###????????";
    //let res = generate_all_combinations(&[3, 2, 1].to_vec(), pattern, String::from(""), &mut seen);
    let pattern = "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?";
    let res = generate_all_combinations(&[1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6].to_vec(), pattern, String::from(""), &mut seen);
    println!("{:?} | {}", res, res.len());
}

#[test]
fn test_find_arrangements() {
    let res = find_arrangements("?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?", &[1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6].to_vec());
    println!("{:?}", res);
}
