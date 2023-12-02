use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut game: i32 = 1;
    let mut sum: i32 = 0;
    let bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(text) = line {
                //p2
                let parts: Vec<&str> = text.split(": ").collect();
                let sets: Vec<&str> = parts[1].split("; ").collect();
                let mut min_balls = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0)
                ]);
                for set in sets {
                    let balls: Vec<&str> = set.split(", ").collect();
                    for ball in balls {
                        let colors: Vec<&str> = ball.split(" ").collect();
                        let num: i32 = colors[0].parse().unwrap();
                        if num > min_balls[colors[1]] {
                            min_balls.insert(colors[1], num);
                        }
                    }
                }
                println!("{}: red {}, green {}, blue {}", game, min_balls["red"], min_balls["green"], min_balls["blue"]);
                let power = min_balls["red"] * min_balls["green"] * min_balls["blue"];
                sum += power;
                
                //p1
                /*
                let parts: Vec<&str> = text.split(": ").collect();
                let sets: Vec<&str> = parts[1].split("; ").collect();
                let mut possible = true;
                for set in sets {
                    let balls: Vec<&str> = set.split(", ").collect();
                    for ball in balls {
                        let colors: Vec<&str> = ball.split(" ").collect();
                        let num: i32 = colors[0].parse().unwrap();
                        //println!("{} {}", num, colors[1]);
                        if num > bag[colors[1]] {
                            println!("{}: {} has {} > {}", game, colors[1], num, bag[colors[1]]);
                            possible = false;
                            break;
                        }
                    }
                    if !possible {
                        break;
                    }
                }
                if possible {
                    //println!("{} is possible", game);
                    sum += game;
                }
                */
            }
            game += 1;
        }
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
