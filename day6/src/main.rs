use roots::find_roots_quadratic;
use roots::Roots;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut time: Vec<i32> = Vec::new();
        let mut distance: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(text) = line {
                //println!("{}", text);
                if text.starts_with("Time:") {
                    if let Some((_, time_str)) = text.split_once(":") {
                        time = time_str
                            .split(" ")
                            .collect::<Vec<&str>>()
                            .iter()
                            .filter(|e| !e.is_empty())
                            .map(|e| e.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                    }
                } else {
                    if let Some((_, distance_str)) = text.split_once(":") {
                        distance = distance_str
                            .split(" ")
                            .collect::<Vec<&str>>()
                            .iter()
                            .filter(|e| !e.is_empty())
                            .map(|e| e.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                    }
                }
            }
        }
        println!("{:?}, {:?}", time, distance);

        //p2
        let mut total_time: String = String::from("");
        let mut total_distance: String = String::from("");
        println!("{}", total_time);
        for num in time {
            total_time.push_str(&(num.to_string()));
        }
        for num in distance {
            total_distance.push_str(&(num.to_string()));
        }
        let total_time: f64 = total_time.parse::<f64>().unwrap();
        let total_distance: f64 = total_distance.parse::<f64>().unwrap();
        println!("time: {} | distance: {}", total_time, total_distance);
        let roots = find_roots_quadratic(-1f64, total_time, -total_distance);
        //println!("{:?}", roots);
        match roots {
            Roots::Two([start, end]) => {
                println!("({}, {})", start, end);
                println!("{}", end.floor() - start.ceil() + 1f64);
            },
            _ => {}
        }

        //p1
        /*
        let mut ways: Vec::<f32> = Vec::new();
        let it = time.iter().zip(distance.iter());
        for (index, (time, record_distance)) in it.enumerate() {
            println!("time: {} | record distance {}", time, record_distance);
            let roots = find_roots_quadratic(-1f32, *time as f32, -record_distance as f32);
            //println!("{:?}", roots);
            match roots {
                Roots::Two([start, end]) => {
                    println!("({}, {})", start, end);
                    let mut adjust: f32 = 0f32;
                    if start == start.ceil() {
                        adjust += 1f32;
                    }
                    if end == end.ceil() {
                        adjust += 1f32;
                    }
                    ways.push(end.floor() - start.ceil() - adjust + 1f32);
                }
                _ => {}
            }
        }
        let mut product: f32 = 1f32;
        for way in &ways {
            product *= *way;
        }
        println!("{:?}, {}", ways, product);
        */
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
