use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    fn advance(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
        }
    }
}

fn main() {
    let mut map: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = read_lines("./test.txt") {
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                map.push(text.chars().map(|c| c.to_digit(10).unwrap()).collect());
            }
        }
    }

    //println!("{:?}", map);
    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut heat_loss: HashMap<(isize, isize), u32> = HashMap::new();
    search(&map, &mut seen, &mut &mut heat_loss, (0, 0), &Direction::Right, 3, 0);

    println!("{}", heat_loss[&((map.len() - 1) as isize, (map[0].len() - 1) as isize)]);
}

fn search(map: &Vec<Vec<u32>>, seen: &mut HashSet<(usize, usize, Direction)>, heat_loss: &mut HashMap<(isize, isize), u32>, pos: (isize, isize), dir: &Direction, blocks_left: usize, pre_sum: u32) {
    let (x, y) = pos;
    if x >= 0 && y >= 0 && (x as usize) < map.len() && (y as usize) < map[0].len() {
        let key = (x as usize, y as usize, dir.clone());
        if seen.contains(&key) {
            let sum = pre_sum + map[x as usize][y as usize];
            if sum < heat_loss[&pos] {
                heat_loss.insert(pos, sum);
                
                if pos == ((map.len() - 1) as isize, (map[0].len() - 1) as isize) {
                    return;
                }

                if blocks_left > 0 {
                    search(map, seen, heat_loss, dir.advance(&pos), dir, blocks_left - 1, sum);
                }
                match dir {
                    Direction::Left | Direction::Right => {
                        search(map, seen, heat_loss, Direction::Up.advance(&pos), &Direction::Up, 3, sum);
                        search(map, seen, heat_loss, Direction::Down.advance(&pos), &Direction::Down, 3, sum);
                    },
                    Direction::Up | Direction::Down => {
                        search(map, seen, heat_loss, Direction::Left.advance(&pos), &Direction::Left, 3, sum);
                        search(map, seen, heat_loss, Direction::Right.advance(&pos), &Direction::Right, 3, sum);
                    }
                }
            }
        }
        else {
            let sum = pre_sum + map[x as usize][y as usize];
            seen.insert(key);
            heat_loss.insert(pos, sum);

            if pos == ((map.len() - 1) as isize, (map[0].len() - 1) as isize) {
                return;
            }

            if blocks_left > 0 {
                search(map, seen, heat_loss, dir.advance(&pos), dir, blocks_left - 1, sum);
            }
            match dir {
                Direction::Left | Direction::Right => {
                    search(map, seen, heat_loss, Direction::Up.advance(&pos), &Direction::Up, 3, sum);
                    search(map, seen, heat_loss, Direction::Down.advance(&pos), &Direction::Down, 3, sum);
                },
                Direction::Up | Direction::Down => {
                    search(map, seen, heat_loss, Direction::Left.advance(&pos), &Direction::Left, 3, sum);
                    search(map, seen, heat_loss, Direction::Right.advance(&pos), &Direction::Right, 3, sum);
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
