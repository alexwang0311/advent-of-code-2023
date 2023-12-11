use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut universe: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                let mut line: Vec<(usize, usize)> = Vec::new();
                for (col, c) in text.chars().into_iter().enumerate() {
                    match c {
                        '#' => {
                            galaxies.insert((row, col));
                        }
                        _ => {}
                    }
                    line.push((row, col));
                }
                universe.push(line);
            }
        }

        //print_universe(&universe, &galaxies);
        //p1
        //expand(&mut universe, &galaxies, 2);
        //p2
        expand(&mut universe, &galaxies, 1_000_000);
        let galaxies = galaxies.into_iter().collect::<Vec<(usize, usize)>>();
        let mut total_distance: usize = 0;
        for i in 0..galaxies.len() {
            let (row_a, col_a) = galaxies[i];
            let (x_a, y_a) = universe[row_a][col_a];
            for j in (i + 1)..galaxies.len() {
                let (row_b, col_b) = galaxies[j];
                let (x_b, y_b) = universe[row_b][col_b];
                let distance: usize = x_a.abs_diff(x_b) + y_a.abs_diff(y_b);
                total_distance += distance;
            }
        }

        println!("{}", total_distance);
    }
}

fn print_universe(universe: &Vec<Vec<(usize, usize)>>, galaxies: &HashSet<(usize, usize)>) {
    for (x, row) in universe.iter().enumerate() {
        for (y, coord) in row.iter().enumerate() {
            if galaxies.contains(&(x, y)) {
                print!("({}, {})", coord.0, coord.1);
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}

fn expand(universe: &mut Vec<Vec<(usize, usize)>>, galaxies: &HashSet<(usize, usize)>, expansion_size: usize) {
    let rows_with_galaxies = galaxies.iter().map(|(x, _)| *x).collect::<HashSet<usize>>();
    let rows_to_expand = (0..universe.len()).into_iter().filter(|x| !rows_with_galaxies.contains(x)).collect::<Vec<usize>>();
    //println!("rows to expand: {:?}", rows_to_expand);

    for row in rows_to_expand {
        let coords = galaxies.iter().filter(|(x, _)| *x > row);
        for (x, y) in coords {
            universe[*x][*y] = (universe[*x][*y].0 + expansion_size - 1, universe[*x][*y].1);
        }
    }

    //print_universe(&universe, &galaxies);

    let cols_with_galaxies = galaxies.iter().map(|(_, y)| *y).collect::<HashSet<usize>>();
    let cols_to_expand = (0..universe[0].len()).into_iter().filter(|y| !cols_with_galaxies.contains(y)).collect::<Vec<usize>>();
    //println!("cols to expand: {:?}", cols_to_expand);

    for col in cols_to_expand {
        let coords = galaxies.iter().filter(|(_, y)| *y > col);
        for (x, y) in coords {
            universe[*x][*y] = (universe[*x][*y].0, universe[*x][*y].1 + expansion_size - 1);
        }
    }

    //print_universe(&universe, &galaxies);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
