use std::collections::HashSet;
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
                    
                    let record = (record.to_owned() + "?").repeat(5);
                    let record = &record[..record.len() - 1];
                    let groups = (groups.to_owned() + ",").repeat(5);
                    let groups = &groups[..groups.len() - 1];
                    
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

fn generate_spring_configurations(sprint_len: usize, total_len: usize) -> Vec<String> {
    let mut configurations: Vec<String> = Vec::new();
    for i in 0..(total_len - sprint_len + 1) {
        let configuration = [vec!['.'; i].as_slice(), vec!['#'; sprint_len].as_slice()].concat();
        for j in 0..(total_len - i - sprint_len + 1) {
            let configuration = [configuration.as_slice(), vec!['.'; j].as_slice()].concat();
            let configuration = configuration.into_iter().collect::<String>();
            configurations.push(configuration);
        }
    }
    configurations
}

//P2 idea: reduce recursion # by forwarding the current sequences into recursive calls
//consider a pattern p = p1, p2, ..., pn; the total comb is comb(p1) * comb(p2) * ... * comb(pn)
fn generate_all_combinations(groups: &Vec<usize>, size: usize) -> HashSet<String> {
    //println!("groups: {:?} | size: {}", groups, size);
    let mut combos: HashSet<String> = HashSet::new();
    if groups.len() == 0 {
        combos.insert(vec!['.'; size].into_iter().collect());
        //println!("nothing | combos: {:?}", combos);
        return combos;
    }

    let first_spring_len = groups[0];
    let rest_len = groups[1..].into_iter().sum::<usize>();
    let rest_count = groups[1..].len();
    let first_section_max_len = size - rest_len - rest_count;
    let first_spring_configurations = generate_spring_configurations(first_spring_len, first_section_max_len);

    for first_configuration in first_spring_configurations {
        //TODO: skip seen combos to avoid dup recursion
        if first_configuration.chars().last().unwrap() == '#' && size > first_configuration.len() {
            let rest_configurations = generate_all_combinations(&groups[1..].to_vec(), size - first_configuration.len() - 1);
            for rest_configuration in rest_configurations {
                combos.insert(format!("{first_configuration}.{rest_configuration}"));
            }
        }
        else {
            let rest_configurations = generate_all_combinations(&groups[1..].to_vec(), size - first_configuration.len());
            for rest_configuration in rest_configurations {
                combos.insert(format!("{first_configuration}{rest_configuration}"));
            }
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
    let combos: HashSet<String> = generate_all_combinations(groups, record.len());
    //println!("all possible combinations: {:?}", combos);
    combos.into_iter().filter(|combo| match_pattern(combo, record)).count()
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
    let res = generate_spring_configurations(3, 7);
    println!("{:?}", res);
}

#[test]
fn test_generate_all_combinations() {
    let res = generate_all_combinations(&[3, 2, 1].to_vec(), 12);
    println!("{:?}", res);
}

#[test]
fn test_find_arrangements() {
    let res = find_arrangements("?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?", &[1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6].to_vec());
    println!("{:?}", res);
}
