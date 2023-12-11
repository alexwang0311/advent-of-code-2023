use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum Space{
    Galaxy((usize, usize)),
    Nothing
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        //p2
        let mut universe: Vec<Vec<Space>> = Vec::new();
        let mut galaxies: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut galaxy_count: usize = 0;
        //p1
        //let mut universe: Vec<Vec<String>> = Vec::new();
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                //p2
                let mut line: Vec<Space> = Vec::new();
                for (col, c) in text.chars().into_iter().enumerate() {
                    match c {
                        '#' => {
                            galaxy_count += 1;
                            galaxies.insert(galaxy_count, (row, col));
                            line.push(Space::Galaxy((row, col)));
                        }
                        _ => {
                            line.push(Space::Nothing);
                        }
                    }
                }
                universe.push(line);
                //p1
                //universe.push(text.chars().map(|c| c.to_string()).collect::<Vec<String>>());
            }
        }

        //p2
        //print_universe_p2(&universe);
        expand_universe_p2(&mut universe, &galaxies, 1_000_000);
        print_universe_p2(&universe);
        let mut total_distance: usize = 0;
        for i in 1..(galaxies.len() + 1) {
            for j in (i + 1)..(galaxies.len() + 1) {
                match (&universe[galaxies[&i].0][galaxies[&i].1], &universe[galaxies[&j].0][galaxies[&j].1]) {
                    (Space::Galaxy((x_0, y_0)), Space::Galaxy((x_1, y_1))) => {
                        let distance: usize = x_0.abs_diff(x_1.clone()) + y_0.abs_diff(y_1.clone());
                        total_distance += distance;
                    },
                    _ => {
                        panic!("unexpected pair - ({}, {}): {:?} | ({}, {}): {:?}", galaxies[&i].0, galaxies[&i].1, universe[galaxies[&i].0][galaxies[&i].1], galaxies[&j].0, galaxies[&j].1, universe[galaxies[&j].0][galaxies[&j].1]);
                    }
                }
            }
        }

        println!("{}", total_distance);

        //p1
        /* 
        //println!("{:?}", universe);
        expand_universe(&mut universe);
        //print_universe(&universe);
        let galaxies = mark_galaxies(&mut universe);
        print_universe(&universe);
        //println!("{:?}", galaxies);
        let mut total_distance: usize = 0;
        for i in 1..(galaxies.len() + 1) {
            for j in (i + 1)..(galaxies.len() + 1) {
                let distance: usize = galaxies[&i].0.abs_diff(galaxies[&j].0) + galaxies[&i].1.abs_diff(galaxies[&j].1);
                //println!("{}: ({}, {}) - {}: ({}, {}) | distance; {}", i, galaxies[&i].0, galaxies[&i].1, j, galaxies[&j].0, galaxies[&j].1, distance);
                total_distance += distance;
            }
        }

        println!("{}", total_distance);
        */
    }
}

fn print_universe_p2(universe: &Vec<Vec<Space>>) {
    for line in universe {
        for space in line {
            match space {
                Space::Galaxy((row, col)) => {
                    print!("({},{})", row, col);
                }
                _ => {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn expand_universe_p2(universe: &mut Vec<Vec<Space>>, galaxies: &HashMap<usize, (usize, usize)>, expansion_size: usize) {
    let mut rows: Vec<usize> = Vec::new();
    for (index, row) in universe.iter().enumerate() {
        if row.iter().filter(|&e| {
            match *e {
                Space::Galaxy(_) => true,
                Space::Nothing => false
            }
        }).count() == 0 {
            rows.push(index);
        }
    }

    //println!("rows to expand: {:?}", rows);
    for (offset, index) in rows.iter().enumerate() {
        let coords = galaxies.iter().filter(|&(k, (x, y))| x > index).map(|(k, (x, y))| (*x, *y)).collect::<Vec<(usize, usize)>>();
        for (row, col) in coords {
            match universe[row][col] {
                Space::Galaxy((x_0, y_0)) => {
                    universe[row][col] = Space::Galaxy((x_0 + expansion_size - 1, y_0));
                },
                _ => {}
            }
        }
    }

    //print_universe_p2(&universe);

    let mut cols: Vec<usize> = Vec::new();
    for col in 0..universe[0].len() {
        let mut expand_col = true;
        for row in 0..universe.len() {
            match universe[row][col] {
                Space::Galaxy(_) => {
                    expand_col = false;
                    break;
                },
                _ => {}
            }
        }

        if expand_col {
            cols.push(col);
        }
    }

    //println!("cols to expand: {:?}", cols);
    for (offset, index) in cols.iter().enumerate() {
        let coords = galaxies.iter().filter(|&(k, (x, y))| y > index).map(|(k, (x, y))| (*x, *y)).collect::<Vec<(usize, usize)>>();
        for (row, col) in coords {
            match universe[row][col] {
                Space::Galaxy((x_0, y_0)) => {
                    universe[row][col] = Space::Galaxy((x_0, y_0 + expansion_size - 1));
                },
                _ => {}
            }
        }
    }

    //print_universe_p2(&universe);
}

fn mark_galaxies(universe: &mut Vec<Vec<String>>) -> HashMap<usize, (usize, usize)> {
    let mut count: u32 = 1;
    let mut hm: HashMap<usize, (usize, usize)> = HashMap::new();
    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == "#" {
                universe[row][col] = count.to_string();
                hm.insert(count as usize, (row, col));
                count += 1;
            }
        }
    }
    hm
}

fn print_universe(universe: &Vec<Vec<String>>) {
    for line in universe {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn expand_universe(universe: &mut Vec<Vec<String>>) {
    let mut rows: Vec<usize> = Vec::new();
    for (index, row) in universe.iter().enumerate() {
        if row.iter().filter(|&e| *e == "#").count() == 0 {
            rows.push(index);
        }
    }

    //println!("rows to expand: {:?}", rows);
    for (offset, index) in rows.iter().enumerate() {
        universe.insert(offset + index, vec![String::from("."); universe[0].len()]);
    }

    let mut cols: Vec<usize> = Vec::new();
    for col in 0..universe[0].len() {
        let mut expand_col = true;
        for row in 0..universe.len() {
            if universe[row][col] == "#" {
                expand_col = false;
                break;
            }
        }

        if expand_col {
            cols.push(col);
        }
    }

    //println!("cols to expand: {:?}", cols);
    for (offset, index) in cols.iter().enumerate() {
        for row in 0..universe.len() {
            universe[row].insert(offset + index, String::from("."));
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
