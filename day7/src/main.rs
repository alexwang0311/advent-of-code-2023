use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::char;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Eq, PartialEq, Debug)]
struct Hand{
    cards: Vec<u32>,
    hand_type: Type,
    bid: u32,
    str: String
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.hand_type.cmp(&self.hand_type) {
            Ordering::Equal => {
                match (self.cards[0].cmp(&other.cards[0]), self.cards[1].cmp(&other.cards[1]), self.cards[2].cmp(&other.cards[2]), self.cards[3].cmp(&other.cards[3]), self.cards[4].cmp(&other.cards[4])) {
                    (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal, _) => self.cards[4].cmp(&other.cards[4]),
                    (Ordering::Equal, Ordering::Equal, Ordering::Equal, _, _) => self.cards[3].cmp(&other.cards[3]),
                    (Ordering::Equal, Ordering::Equal, _, _, _) => self.cards[2].cmp(&other.cards[2]),
                    (Ordering::Equal, _, _, _, _) => self.cards[1].cmp(&other.cards[1]),
                    (_, _, _, _, _) => self.cards[0].cmp(&other.cards[0])
                }
            },
            _ => other.hand_type.cmp(&self.hand_type)
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let clock = Instant::now();

        let mut entries: Vec<Hand> = Vec::new();
        for line in lines {
            if let Ok(text) = line {
                if let Some((cards, bid)) = text.split_once(" ") {
                    //p2
                    if let Some(hand) = build_hand_p2(cards, bid.parse::<u32>().unwrap()) {
                    //p1
                    //if let Some(hand) = build_hand(cards, bid.parse::<u32>().unwrap()) {
                        //println!("cards: {}, bid: {} | hand: {:?}", cards, bid, hand);
                        entries.push(hand);
                    }
                    else {
                        panic!("failed to parse hand: {} | {}", cards, bid);
                    }
                }
            }
        }
        entries.sort();
        //println!("cards: {:?}", entries.iter().map(|e| &e.str).collect::<Vec<&String>>());
        //println!("bids: {:?}", entries.iter().map(|e| usize::try_from(e.bid).unwrap()).collect::<Vec<usize>>());
        
        let winning = entries.iter()
                                    .map(|e| usize::try_from(e.bid).unwrap())
                                    .enumerate()
                                    .fold(0, |sum, (index, bid)| sum + (index + 1) * bid);
        println!("ans: {} | time {:?}", winning, clock.elapsed());
    }
}

//p2
fn char_to_u32_p2(char: &char) -> u32 {
    if char.is_digit(10) {
        return char.to_digit(10).unwrap()
    }

    match char {
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}

fn build_hand_p2(str: &str, bid: u32) -> Option<Hand> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    let mut j_count: u32 = 0;
    for char in str.chars() {
        if char != 'J' {
            let val = hm.entry(char).or_insert(0);
            *val += 1;
        }
        else {
            j_count += 1;
        }
    }

    let cards = str.chars().into_iter().map(|c| char_to_u32_p2(&c)).collect::<Vec<u32>>();

    match hm.keys().len() {
        5 => Some(Hand {
            hand_type: Type::HighCard,
            cards: cards,
            bid: bid,
            str: str.to_owned()
        }),
        4 => Some(Hand {
            hand_type: Type::OnePair,
            cards: cards,
            bid: bid,
            str: str.to_owned()
        }),
        3 => {
            match j_count {
                1 | 2 => Some(Hand {
                    hand_type: Type::ThreeOfAKind,
                    cards: cards,
                    bid: bid,
                    str: str.to_owned()
                }),
                _ => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Hand {
                            hand_type: Type::ThreeOfAKind,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                    else {
                        Some(Hand {
                            hand_type: Type::TwoPair,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                }
            }
        },
        2 => {
            match j_count {
                1 => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Hand {
                            hand_type: Type::FourOfAKind,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                    else {
                        Some(Hand {
                            hand_type: Type::FullHouse,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                },
                2 | 3 => Some(Hand {
                    hand_type: Type::FourOfAKind,
                    cards: cards,
                    bid: bid,
                    str: str.to_owned()
                }),
                _ => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Hand {
                            hand_type: Type::FullHouse,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                    else {
                        Some(Hand {
                            hand_type: Type::FourOfAKind,
                            cards: cards,
                            bid: bid,
                            str: str.to_owned()
                        })
                    }
                }
            }
        },
        //NOTE: spent waaaaaaaay too much time figuring out edge case 'JJJJJ' -> 0 key
        1 | 0 => Some(Hand {
            hand_type: Type::FiveOfAKind,
            cards: cards,
            bid: bid,
            str: str.to_owned()
        }),
        _ => None
    }
}

//p1
/*
fn char_to_u32(char: &char) -> u32 {
    if char.is_digit(10) {
        return char.to_digit(10).unwrap()
    }

    match char {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}

fn build_hand(str: &str, bid: u32) -> Option<Hand> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    for char in str.chars() {
        let val = hm.entry(char).or_insert(0);
        *val += 1;
    }
    let cards = str.chars().into_iter().map(|c| char_to_u32(&c)).collect::<Vec<u32>>();

    match hm.keys().len() {
        5 => {
            Some(Hand {
                hand_type: Type::HighCard,
                bid: bid,
                cards: cards,
                str: str.to_owned()
            })
        },
        4 => {
            Some(Hand {
                hand_type: Type::OnePair,
                bid: bid,
                cards: cards,
                str: str.to_owned()
            })
        },
        3 => {
            if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                Some(Hand {
                    hand_type: Type::ThreeOfAKind,
                    bid: bid,
                    cards: cards,
                    str: str.to_owned()
                })
            }
            else {
                Some(Hand {
                    hand_type: Type::TwoPair,
                    bid: bid,
                    cards: cards,
                    str: str.to_owned()
                })
            }
        },
        2 => {
            if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                Some(Hand {
                    hand_type: Type::FullHouse,
                    bid: bid,
                    cards: cards,
                    str: str.to_owned()
                })
            }
            else {
                Some(Hand {
                    hand_type: Type::FourOfAKind,
                    bid: bid,
                    cards: cards,
                    str: str.to_owned()
                })
            }

        },
        1 => {
            Some(Hand {
                hand_type: Type::FiveOfAKind,
                bid: bid,
                cards: cards,
                str: str.to_owned()
            })
        }
        _ => None
    }
}
*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test() {
    let entry1 = build_hand_p2("JJ8K8", 100).unwrap();
    let entry2 = build_hand_p2("J222K", 100).unwrap();
    let entry3 = build_hand_p2("J23K5", 100).unwrap();
    //println!("{:?}, {:?}, {}", entry2, entry3, entry2 < entry3);
    let mut entries = Vec::new();
    entries.push(entry1);
    entries.push(entry2);
    entries.push(entry3);
    entries.sort();
    println!("{:?}", entries);
}