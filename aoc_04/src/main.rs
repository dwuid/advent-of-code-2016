
#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;

macro_rules! some {
    ($expression:expr) => (match $expression {
        Some(value) => value,
        None => return None,
    })
}

fn most_common(name: &str) -> String {
    name.chars()
            .filter(|&x| x != '-')
            .sorted()
            .into_iter()
            .group_by(|&x| x)
            .into_iter()
            .map(|(k, v)| (k, v.count()))
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .into_iter()
            .map(|x| x.0)
            .take(5)
            .collect::<String>()[..5]
        .to_string()
}

struct Room {
    name: String,
    sector_id: usize,
    is_valid: bool,
}

fn parse_room(name: &str) -> Option<Room> {
    lazy_static! {
        static ref ROOM: Regex = Regex::new(r"((?:[a-z]+-)+)(\d+)\[([a-z]+)\]")
            .unwrap();
    }

    let captures = match ROOM.captures(name) {
        Some(x) => x,
        _ => return None,
    };

    let name = some!(captures.at(1));
    let name = &name[..(name.len() - 1)];

    let sector_id: usize = some!(captures.at(2).and_then(|x| x.parse().ok()));
    let checksum = some!(captures.at(3));

    let room = Room {
        name: name.to_string(),
        sector_id: sector_id,
        is_valid: most_common(name) == checksum,
    };

    Some(room)
}

fn solve_one(rooms: &[Room]) -> usize {
    rooms.iter().filter(|r| r.is_valid).fold(0, |acc, r| acc + r.sector_id)
}

fn solve_two(rooms: &[Room]) -> Option<usize> {
    const BASE: u8 = b'a';

    for room in rooms.iter().filter(|r| r.is_valid) {
        let key = (room.sector_id % 26) as u8;
        let decrypted = room.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => ((c as u8 + key - BASE) % 26 + BASE) as char,
            })
            .collect::<String>();

        if decrypted.contains("north") {
            return Some(room.sector_id);
        }
    }

    None
}

fn main() {
    let rooms: Vec<_> = include_str!("../input")
        .trim()
        .lines()
        .filter_map(|r| parse_room(r))
        .collect();

    println!("[1] The sum of valid rooms is {}.", solve_one(&rooms));

    match solve_two(&rooms) {
        Some(id) => println!("[2] The objects are stored in room {}.", id),
        _ => println!("[2] Cannot find the room where the objects are stored."),
    }
}

