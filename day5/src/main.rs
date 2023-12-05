use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;
use std::collections::VecDeque;

#[derive(Debug)]
struct Entry {
    src: u64,
    dst: u64,
    range: u64
}

#[derive(Debug, Copy, Clone)]
struct Range(u64, u64);

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut seeds: Vec<u64> = Vec::new();
        let mut maps: HashMap<String, Vec<Entry>> = HashMap::new();
        let mut cur_map: Option<String> = None;
        for line in lines {
            if let Ok(text) = line {
                if !text.is_empty() {
                    if let Some((_, seeds_str)) = text.split_once(": ") {
                        seeds = seeds_str.split(" ").collect::<Vec<&str>>().iter().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                    }
                    else if let Some((map_name, _)) = text.split_once(" map:") {
                        //println!("{}", map_name);
                        cur_map = Some(String::from(map_name));
                        //println!("{:?}", cur_map);
                        maps.insert(map_name.to_owned(), Vec::new());
                    }
                    else {
                        if let Some(ref name) = cur_map {
                            //println!("{}: {}", name, text);
                            if let Some(map) = maps.get_mut(name) {
                                let entry: Vec<u64> = text.split(" ").collect::<Vec<&str>>().iter().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                                let entry = Entry {
                                    dst: entry[0],
                                    src: entry[1],
                                    range: entry[2]
                                };
                                //println!("{:?}", entry);
                                map.push(entry);
                                //println!("{:?}", map);
                            }
                        }
                    }
                    
                }
            }
        }
        //println!("{:?}", seeds);
        for (_, map) in &mut maps {
            map.sort_by(|a, b| a.src.cmp(&b.src));
            //println!("{:?}", map);
        }
        //p2
        let mut ranges: Vec<Range> = Vec::new();
        let mut i: usize = 0;
        while i < seeds.len() {
            ranges.push(Range(seeds[i], seeds[i] + seeds[i + 1] - 1));
            i += 2;
        }
        //println!("{:?}", ranges);
        let mut map_name: Vec<&String> = maps.keys().filter(|key| key.starts_with("seed")).collect::<Vec<&String>>();
        while map_name.len() > 0 {
            println!("Ranges: {:?}", ranges);
            let name = map_name[0];
            let mut new_ranges: Vec<Range> = Vec::new();
            for range in ranges {
                let mut transformed = transform_range(&range, &maps[name]);
                println!("from {:?} to {:?}", range, transformed);
                new_ranges.append(&mut transformed);
            }
            ranges = new_ranges;
            if let Some((_, to)) = name.split_once("-to-") {
                map_name = maps.keys().filter(|key| key.starts_with(to)).collect::<Vec<&String>>();
            }
            println!("--------------------------------");
        }
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        println!("final ranges: {:?}", ranges);

        /*
        //p1
        let mut pq: DoublePriorityQueue<u64, u64> = DoublePriorityQueue::new();
        let mut li: Vec<u64> = Vec::new();
        for seed in seeds {
            let mut val: u64 = seed;
            let mut map_name: Vec<&String> = maps.keys().filter(|key| key.starts_with("seed")).collect::<Vec<&String>>();
            while map_name.len() > 0 {
                let map = map_name[0];
                for entry in &maps[map] {
                    if (entry.src..(entry.src + entry.range)).contains(&val) {
                        val = entry.dst + (val - entry.src);
                        break;
                    }
                }
                //println!("{} - {}: {}", seed, map, val);
                if let Some((from, to)) = map.split_once("-to-") {
                    map_name = maps.keys().filter(|key| key.starts_with(to)).collect::<Vec<&String>>();
                }
            }
            li.push(val);
            //pq.push(seed, val);
        }

        li.sort();
        println!("{:?}", li);
        //println!("{:?}", pq);
        */
    }
}

//NOTE: this assumes the map is sorted
fn transform_range(range: &Range, map: &Vec<Entry>) -> Vec<Range> {
    //println!("{:?}, {:?}", range, map);
    let mut transformed: Vec<Range> = Vec::new();
    let mut ranges_to_process: VecDeque<Range> = VecDeque::from([*range]);
    while let Some(range) = ranges_to_process.pop_front() {
        //println!("{:?}, {:?}", range, map);
        //TODO: this is ugly af
        for (index, entry) in map.iter().enumerate() {
            if range.1 < entry.src {
                println!("({}, {}) is to the left of ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                transformed.push(Range(range.0, range.1));
                break;
            }
            
            if range.1 >= entry.src {
                if range.1 <= entry.src + entry.range {
                    if range.0 < entry.src {
                        println!("({}, {}) left intersects ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                        transformed.push(Range(range.0, entry.src - 1));
                        transformed.push(Range(entry.dst, entry.dst + range.1 - entry.src));
                        break;
                    }
                    else {
                        println!("({}, {}) is inside ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                        transformed.push(Range(entry.dst + range.0 - entry.src, entry.dst + range.1 - entry.src));
                        break;
                    }
                }
                else {
                    if range.0 < entry.src {
                        println!("({}, {}) includes ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                        transformed.push(Range(range.0, entry.src - 1));
                        transformed.push(Range(entry.dst, entry.dst + entry.range - 1));
                        break;
                    }
                    else if range.0 < entry.src + entry.range {
                        println!("({}, {}) right intersects ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                        ranges_to_process.push_back(Range(entry.src + entry.range, range.1));
                        println!("adding ({}, {}) for processing", entry.src + entry.range, range.1);
                        transformed.push(Range(entry.dst + range.0 - entry.src, entry.dst + entry.range - 1));
                        break;
                    }
                    else {
                        if index == map.len() - 1 {
                            println!("({}, {}) is to the right of ({}, {})", range.0, range.1, entry.src, entry.src + entry.range - 1);
                            transformed.push(Range(range.0, range.1));
                        }
                    }
                }

            }
        }
    }

    transformed
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}