use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{cmp, char};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
struct Hand{
    cards: Vec<u32>
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.cards[0].cmp(&other.cards[0]), self.cards[1].cmp(&other.cards[1]), self.cards[2].cmp(&other.cards[2]), self.cards[3].cmp(&other.cards[3]), self.cards[4].cmp(&other.cards[4])) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal, _) => self.cards[4].cmp(&other.cards[4]),
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, _, _) => self.cards[3].cmp(&other.cards[3]),
            (Ordering::Equal, Ordering::Equal, _, _, _) => self.cards[2].cmp(&other.cards[2]),
            (Ordering::Equal, _, _, _, _) => self.cards[1].cmp(&other.cards[1]),
            (_, _, _, _, _) => self.cards[0].cmp(&other.cards[0])
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Type {
    FiveOfAKind(Hand),
    FourOfAKind(Hand),
    FullHouse(Hand),
    ThreeOfAKind(Hand),
    TwoPair(Hand),
    OnePair(Hand),
    HighCard(Hand)
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Type::FiveOfAKind(val), Type::FiveOfAKind(other_val)) => val.cmp(other_val),
            (Type::FiveOfAKind(_), _) => Ordering::Greater,
            (Type::FourOfAKind(_), Type::FiveOfAKind(_)) => Ordering::Less,
            (Type::FourOfAKind(val), Type::FourOfAKind(other_val)) => val.cmp(other_val),
            (Type::FourOfAKind(_), _) => Ordering::Greater,
            (Type::FullHouse(_), Type::FiveOfAKind(_)) => Ordering::Less,
            (Type::FullHouse(_), Type::FourOfAKind(_)) => Ordering::Less,
            (Type::FullHouse(val), Type::FullHouse(other_val)) => val.cmp(other_val),
            (Type::FullHouse(_), _) => Ordering::Greater,
            (Type::ThreeOfAKind(_), Type::FiveOfAKind(_)) => Ordering::Less,
            (Type::ThreeOfAKind(_), Type::FourOfAKind(_)) => Ordering::Less,
            (Type::ThreeOfAKind(_), Type::FullHouse(_)) => Ordering::Less,
            (Type::ThreeOfAKind(val), Type::ThreeOfAKind(other_val)) => val.cmp(other_val),
            (Type::ThreeOfAKind(_), _) => Ordering::Greater,
            (Type::TwoPair(_), Type::FiveOfAKind(_)) => Ordering::Less,
            (Type::TwoPair(_), Type::FourOfAKind(_)) => Ordering::Less,
            (Type::TwoPair(_), Type::FullHouse(_)) => Ordering::Less,
            (Type::TwoPair(_), Type::ThreeOfAKind(_)) => Ordering::Less,
            (Type::TwoPair(val), Type::TwoPair(other_val)) => val.cmp(other_val),
            (Type::TwoPair(_), _) => Ordering::Greater,
            (Type::OnePair(_), Type::FiveOfAKind(_)) => Ordering::Less,
            (Type::OnePair(_), Type::FourOfAKind(_)) => Ordering::Less,
            (Type::OnePair(_), Type::FullHouse(_)) => Ordering::Less,
            (Type::OnePair(_), Type::ThreeOfAKind(_)) => Ordering::Less,
            (Type::OnePair(_), Type::TwoPair(_)) => Ordering::Less,
            (Type::OnePair(val), Type::OnePair(other_val)) => val.cmp(other_val),
            (Type::OnePair(_), _) => Ordering::Greater,
            (Type::HighCard(val), Type::HighCard(other_val)) => val.cmp(other_val),
            (Type::HighCard(_), _) => Ordering::Less
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    hand_type: Type,
    bid: u32,
    cards: String
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut entries: Vec<Entry> = Vec::new();
        for line in lines {
            if let Ok(text) = line {
                if let Some((cards, bid)) = text.split_once(" ") {
                    //println!("{}, {}", cards, bid);
                    //p2
                    if let Some(hand) = build_hand_p2(cards, bid.parse::<u32>().unwrap()) {
                    //p1
                    //if let Some(hand) = build_hand(cards, bid.parse::<u32>().unwrap()) {
                        //println!("{:?}", hand);
                        entries.push(hand);
                    }
                }
            }
        }
        entries.sort();
        //let entries = entries.iter().map(|e| &e.cards).collect::<Vec<&String>>();
        //println!("{:?}", entries);
        let bids = entries.iter().map(|e| usize::try_from(e.bid).unwrap()).collect::<Vec<usize>>();
        //println!("{:?}", bids);
        
        let mut winning: usize = 0;
        for (index , bid) in bids.iter().enumerate() {
            winning += *bid * (index + 1);
        }
        println!("{}", winning);
        
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

fn build_hand_p2(str: &str, bid: u32) -> Option<Entry> {
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
    let hand = Hand {
        cards: cards
    };

    match hm.keys().len() {
        5 => {
            Some(Entry {
                hand_type: Type::HighCard(hand),
                bid: bid,
                cards: str.to_owned()
            })
        },
        4 => Some(Entry {
            hand_type: Type::OnePair(hand),
            bid: bid,
            cards: str.to_owned()
        }),
        3 => {
            match j_count {
                1 | 2 => Some(Entry {
                    hand_type: Type::ThreeOfAKind(hand),
                    bid: bid,
                    cards: str.to_owned()
                }),
                _ => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Entry {
                            hand_type: Type::ThreeOfAKind(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                    else {
                        Some(Entry {
                            hand_type: Type::TwoPair(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                }
            }
        },
        2 => {
            match j_count {
                1 => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Entry {
                            hand_type: Type::FourOfAKind(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                    else {
                        Some(Entry {
                            hand_type: Type::FullHouse(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                },
                2 | 3 => Some(Entry {
                    hand_type: Type::FourOfAKind(hand),
                    bid: bid,
                    cards: str.to_owned()
                }),
                _ => {
                    if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                        Some(Entry {
                            hand_type: Type::FullHouse(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                    else {
                        Some(Entry {
                            hand_type: Type::FourOfAKind(hand),
                            bid: bid,
                            cards: str.to_owned()
                        })
                    }
                }
            }
        },
        1 | 0 => Some(Entry {
            hand_type: Type::FiveOfAKind(hand),
            bid: bid,
            cards: str.to_owned()
        }),
        _ => None
    }
}

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

fn build_hand(str: &str, bid: u32) -> Option<Entry> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    for char in str.chars() {
        let val = hm.entry(char).or_insert(0);
        *val += 1;
    }
    let cards = str.chars().into_iter().map(|c| char_to_u32(&c)).collect::<Vec<u32>>();
    let hand = Hand {
        cards: cards
    };
    match hm.keys().len() {
        5 => {
            Some(Entry {
                hand_type: Type::HighCard(hand),
                bid: bid
            })
        },
        4 => {
            Some(Entry {
                hand_type: Type::OnePair(hand),
                bid: bid
            })
        },
        3 => {
            if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                Some(Entry {
                    hand_type: Type::ThreeOfAKind(hand),
                    bid: bid
                })
            }
            else {
                Some(Entry {
                    hand_type: Type::TwoPair(hand),
                    bid: bid
                })
            }
        },
        2 => {
            if let Some(_) = hm.keys().into_iter().filter(|key| hm[key] == 3).collect::<Vec<_>>().get(0) {
                Some(Entry {
                    hand_type: Type::FullHouse(hand),
                    bid: bid
                })
            }
            else {
                Some(Entry {
                    hand_type: Type::FourOfAKind(hand),
                    bid: bid
                })
            }

        },
        1 => {
            Some(Entry {
                hand_type: Type::FiveOfAKind(hand),
                bid: bid
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