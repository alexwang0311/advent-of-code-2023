use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
use colored::{Colorize, ColoredString, Color};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe{
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Start,
    None
}

impl Pipe {
    fn to_colored_str(&self) -> ColoredString {
        match self {
            Pipe::NS => "│".red().bold(),
            Pipe::EW => "─".red().bold(),
            Pipe::NE => "└".red().bold(),
            Pipe::NW => "┘".red().bold(),
            Pipe::SE => "┌".red().bold(),
            Pipe::SW => "┐".red().bold(),
            Pipe::Start => "S".red().bold(),
            Pipe::None => ".".yellow().bold()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
    None
}

impl From<(isize, isize)> for Direction {
    fn from(value: (isize, isize)) -> Self {
        match value {
            (1, 0) => Direction::S,
            (-1, 0) => Direction::N,
            (0, 1) => Direction::E,
            (0, -1) => Direction::W,
            (_, _) => Direction::None
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::N => write!(f, "↑"),
            Direction::S => write!(f, "↓"),
            Direction::E => write!(f, "→"),
            Direction::W => write!(f, "←"), 
            Direction::None => write!(f, "x")
        }
    }
}

impl Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1), 
            Direction::None => (0, 0)
        }   
    }
}

const NO_DIR: Pipe = Pipe::None;
const HOR_DIR: Pipe = Pipe::EW;
const VER_DIR: Pipe = Pipe::NS;
const NE_DIR: Pipe = Pipe::NE;
const NW_DIR: Pipe = Pipe::NW;
const SE_DIR: Pipe = Pipe::SE;
const SW_DIR: Pipe = Pipe::SW;
const ALL_DIR: Pipe = Pipe::Start;

impl Pipe {
    fn change_dir(&self, dir: &Direction) -> Direction {
        match (self, dir) {
            (Pipe::SW, Direction::N) => Direction::W,
            (Pipe::SW, Direction::E) => Direction::S,
            (Pipe::NW, Direction::S) => Direction::W,
            (Pipe::NW, Direction::E) => Direction::N,
            (Pipe::SE, Direction::N) => Direction::E,
            (Pipe::SE, Direction::W) => Direction::S,
            (Pipe::NE, Direction::S) => Direction::E,
            (Pipe::NE, Direction::W) => Direction::N,
            (Pipe::NS, Direction::N) => Direction::N,
            (Pipe::NS, Direction::S) => Direction::S,
            (Pipe::EW, Direction::E) => Direction::E,
            (Pipe::EW, Direction::W) => Direction::W,
            (Pipe::Start, _) => Direction::from(dir.to_tuple()),
            (_, _) => Direction::None
        }
    }

    fn is_empty_space(&self) -> bool {
        *self == Pipe::None
    }
}

fn main() {
    let file_name = "./input.txt";
    if let Ok(lines) = read_lines(file_name) {
        let mut map: Vec<Vec<Pipe>> = Vec::new();
        let mut starting_pos: Option<(usize, usize)> = None;
        for (row, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                //println!("{}", text);
                let mut pipes: Vec<Pipe> = Vec::new();
                for (col, c) in text.chars().into_iter().enumerate() {
                    let mut pipe: Option<Pipe> = None;
                    match c {
                        'S' => {
                            starting_pos = Some((row, col));
                            pipe = Some(ALL_DIR)
                        },
                        '.' => pipe = Some(NO_DIR),
                        '-' => pipe = Some(HOR_DIR),
                        '|' => pipe = Some(VER_DIR),
                        'L' => pipe = Some(NE_DIR),
                        'J' => pipe = Some(NW_DIR),
                        '7' => pipe = Some(SW_DIR),
                        'F' => pipe = Some(SE_DIR),
                        _ => {}
                    }

                    match pipe {
                        Some(pipe) => pipes.push(pipe),
                        None => panic!("failed to parse {} at ({}, {})", c, row, col)
                    }
                }
                map.push(pipes);
            }
        }

        //println!("{:?}", map);
        match starting_pos {
            Some(starting_pos) => {
                //p2
                let mut pipe: Vec<(Pipe, (usize, usize))> = build_pipe(&map, starting_pos);
                replace_start(&mut pipe);
                render_pipe(&pipe, map.len(), map[0].len());
                let mut area = vec![vec![true; map[0].len()]; map.len()];
                for (p, (x, y)) in &pipe {
                    area[*x][*y] = false;
                }
                let mut count: usize = 0;
                for (row, line) in map.iter().enumerate() {
                    for (col, c) in line.iter().enumerate() {
                        if area[row][col] {
                            let left = pipe.iter().filter(|(_, (x, y))| *x == row && *y < col).filter(|(p, _)| *p == Pipe::NS || *p == Pipe::SW || *p == Pipe::SE).map(|(p, _)| p.to_colored_str()).collect::<Vec<ColoredString>>();
                            let right = pipe.iter().filter(|(_, (x, y))| *x == row && *y > col).filter(|(p, _)| *p == Pipe::NS || *p == Pipe::NW || *p == Pipe::NE).map(|(p, _)| p.to_colored_str()).collect::<Vec<ColoredString>>();
                            /*
                            print!("left: ");
                            print_colored_vec(&left);
                            println!();
                            print!("right: ");
                            print_colored_vec(&right);
                            println!("-------------");
                            */

                            if left.len().min(right.len()) % 2 == 1 {
                                //println!("({}, {}) is inside", row, col);
                                count += 1;
                            }
                        }
                    }
                }

                println!("{}", count);
                
                //p1
                /*
                let steps: usize = traverse_pipe(&map, starting_pos);
                println!("total: {}, furthest: {}", steps, steps / 2);
                */
            },
            None => panic!("did not find starting position")
        }
    }
    else {
        panic!("failed to read file: {}", file_name);
    }
}

fn print_colored_vec(vec: &Vec<ColoredString>) {
    for cs in vec {
        print!("{} ", cs);
    }
    println!();
}


fn render_pipe(pipe: &Vec<(Pipe, (usize, usize))>, row: usize, col:usize) {
    let mut area = vec![vec!["0".yellow(); col]; row];
    for (p, (x, y)) in pipe {
        area[*x][*y] = p.to_colored_str();
    }
    
    for line in area {
        for str in line {
            print!("{}", str);
        }
        println!();
    }
}

fn replace_start(pipe: &mut Vec<(Pipe, (usize, usize))>) {
    let (_, (x_last, y_last)) = pipe[1];
    let (_, (x_1, y_1)) = *pipe.last().unwrap();
    let x_last = isize::try_from(x_last).unwrap();
    let y_last = isize::try_from(y_last).unwrap();
    let x_1 = isize::try_from(x_1).unwrap();
    let y_1 = isize::try_from(y_1).unwrap();
    //println!("({}, {}) - ({}, {}) = ({}, {})", x_1, y_1, x_last, y_last, x_1 - x_last, y_1 - y_last);
    let mut first = &mut pipe[0];

    let dir = (x_1 - x_last, y_1 - y_last);
    match dir {
        (2, 0) | (-2, 0) => first.0 = Pipe::NS,
        (0, 2) | (0, -2) => first.0 = Pipe::EW,
        (-1, 1) => first.0 = Pipe::SE,
        (-1, -1) => first.0 = Pipe::SW,
        (1, -1) => first.0 = Pipe::NW,
        (1, 1) => first.0 = Pipe::NE,
        _ => {}
    }
}

fn find_first_pipe(map: &Vec<Vec<Pipe>>, starting_pos: (usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = starting_pos;
    if x > 0 {
        match map[x - 1][y] {
            Pipe::NS | Pipe::SE | Pipe::SW => return Some((x - 1, y)),
            _ => {}
        }
    }

    if x < map.len() {
        match map[x + 1][y] {
            Pipe::NS | Pipe::NE | Pipe::NW => return Some((x + 1, y)),
            _ => {}
        }
    }

    if y > 0 {
        match map[x][y - 1] {
            Pipe::EW | Pipe::NE | Pipe::SE => return Some((x, y - 1)),
            _ => {}
        }
    }

    if y < map[0].len() {
        match map[x][y + 1] {
            Pipe::EW | Pipe::NW | Pipe::SW => return Some((x, y + 1)),
            _ => {}
        }
    }

    None
}

fn build_pipe(map: &Vec<Vec<Pipe>>, starting_pos: (usize, usize)) -> Vec<(Pipe, (usize, usize))> {
    let mut pipe: Vec<(Pipe, (usize, usize))> = Vec::new();
    pipe.push((map[starting_pos.0][starting_pos.1], starting_pos));

    let first_pipe_coord = find_first_pipe(map, starting_pos).unwrap();
    let mut row = first_pipe_coord.0;
    let mut col = first_pipe_coord.1;
    let dir = (row as isize - starting_pos.0 as isize, col as isize - starting_pos.1 as isize);
    let mut dir = Direction::from(dir);
    //println!("pos: ({}, {}) | dir: {} | starting at: {:?}", row, col, dir, starting_pos);

    while (row, col) != starting_pos {
        pipe.push((map[row][col], (row, col)));
        let new_dir = map[row][col].change_dir(&dir);
        let (x, y) = new_dir.to_tuple();
        let new_row = (isize::try_from(row).unwrap() + x) as usize;
        let new_col = (isize::try_from(col).unwrap() + y) as usize;
        row = new_row;
        col = new_col;
        dir = new_dir;
        //println!("pos: ({}, {}) | dir: {}", row, col, dir);
    }

    pipe
}

fn traverse_pipe(map: &Vec<Vec<Pipe>>, starting_pos: (usize, usize)) -> usize {
    let mut steps: usize = 0;
    let first_pipe_coord = find_first_pipe(map, starting_pos).unwrap();
    let mut row = first_pipe_coord.0;
    let mut col = first_pipe_coord.1;
    let dir = (row as isize - starting_pos.0 as isize, col as isize - starting_pos.1 as isize);
    let mut dir = Direction::from(dir);
    println!("pos: ({}, {}) | dir: {} | starting at: {:?}", row, col, dir, starting_pos);

    while (row, col) != starting_pos {
        let new_dir = map[row][col].change_dir(&dir);
        let (x, y) = new_dir.to_tuple();
        let new_row = (isize::try_from(row).unwrap() + x) as usize;
        let new_col = (isize::try_from(col).unwrap() + y) as usize;
        row = new_row;
        col = new_col;
        dir = new_dir;
        println!("pos: ({}, {}) | dir: {}", row, col, dir);
        steps += 1;
    }

    steps + 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
