use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                map.push(text.chars().collect());
            }
        }
    }

    //println!("{:?}", map);
    //p2
    let mut max: usize = 0;
    for i in 0..map[0].len() {
        //from top
        let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
        ray_march(&map, &mut seen, 0, i as isize, &Direction::Down);
        let count = seen.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(usize, usize)>>().len();
        max = max.max(count);

        //from bot
        seen.clear();
        ray_march(&map, &mut seen, (map.len() - 1) as isize, i as isize, &Direction::Up);
        let count = seen.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(usize, usize)>>().len();
        max = max.max(count);
    }

    for i in 0..map.len() {
        //from left
        let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
        ray_march(&map, &mut seen, i as isize, 0, &Direction::Right);
        let count = seen.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(usize, usize)>>().len();
        max = max.max(count);

        //from right
        seen.clear();
        ray_march(&map, &mut seen, i as isize, (map[0].len() - 1) as isize, &Direction::Left);
        let count = seen.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(usize, usize)>>().len();
        max = max.max(count);
    }

    println!("{}", max);

    //p1
    /* 
    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();

    ray_march(&map, &mut seen, 0, 0, &Direction::Right);

    let mut energized = vec![vec!['.'; map[0].len()]; map.len()];
    let mut count: usize = 0;

    for tuple in seen {
        let x = tuple.0;
        let y = tuple.1;
        if energized[x][y] == '.' {
            energized[x][y] = '#';
            count += 1;
        }
    }

    for v in energized {
        println!("{}", v.iter().collect::<String>());
    }

    println!("{}", count);
    */
}

fn ray_march(map: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize, Direction)>, x: isize, y: isize, dir: &Direction) {
    if x >= 0 && y >= 0 && (x as usize) < map.len() && (y as usize) < map[0].len() && !seen.contains(&(x as usize, y as usize, dir.clone())) {
        seen.insert((x as usize, y as usize, *dir));
        let tile = map[x as usize][y as usize];
        match dir {
            Direction::Right => {
                match tile {
                    '.' | '-' => ray_march(map, seen, x, y + 1, dir),
                    '\\' => ray_march(map, seen, x + 1, y, &Direction::Down),
                    '/' => ray_march(map, seen, x - 1, y, &Direction::Up),
                    '|' => {
                        ray_march(map, seen, x - 1, y, &Direction::Up);
                        ray_march(map, seen, x + 1, y, &Direction::Down);
                    },
                    _ => panic!("unexpected tile at ({}, {}): {}", x, y, tile)
                }
            },
            Direction::Left => {
                match tile {
                    '.' | '-' => ray_march(map, seen, x, y - 1, dir),
                    '\\' => ray_march(map, seen, x - 1, y, &Direction::Up),
                    '/' => ray_march(map, seen, x + 1, y, &Direction::Down),
                    '|' => {
                        ray_march(map, seen, x - 1, y, &Direction::Up);
                        ray_march(map, seen, x + 1, y, &Direction::Down);
                    },
                    _ => panic!("unexpected tile at ({}, {}): {}", x, y, tile)
                }
            },
            Direction::Up => {
                match tile {
                    '.' | '|' => ray_march(map, seen, x - 1, y, dir),
                    '\\' => ray_march(map, seen, x, y - 1, &Direction::Left),
                    '/' => ray_march(map, seen, x, y + 1, &Direction::Right),
                    '-' => {
                        ray_march(map, seen, x, y - 1, &Direction::Left);
                        ray_march(map, seen, x, y + 1, &Direction::Right);
                    },
                    _ => panic!("unexpected tile at ({}, {}): {}", x, y, tile)
                }
            },
            Direction::Down => {
                match tile {
                    '.' | '|' => ray_march(map, seen, x + 1, y, dir),
                    '/' => ray_march(map, seen, x, y - 1, &Direction::Left),
                    '\\' => ray_march(map, seen, x, y + 1, &Direction::Right),
                    '-' => {
                        ray_march(map, seen, x, y - 1, &Direction::Left);
                        ray_march(map, seen, x, y + 1, &Direction::Right);
                    },
                    _ => panic!("unexpected tile at ({}, {}): {}", x, y, tile)
                }
            }
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
