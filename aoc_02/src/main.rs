use std::ops::{Index, IndexMut};

// I assumed part 2 would require generalizing over the number of the keypad.
// Turns out I was wrong! :)
const N: usize = 3;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,

    Count = 4,
}

type Grid = [usize; N * N];
type Traversal = [Grid; Direction::Count as usize];

impl Index<Direction> for Traversal {
    type Output = Grid;

    fn index<'a>(&'a self, index: Direction) -> &'a Grid {
        &self[index as usize]
    }
}

impl IndexMut<Direction> for Traversal {
    fn index_mut<'a>(&'a mut self, index: Direction) -> &'a mut Grid {
        &mut self[index as usize]
    }
}

fn build_traversal() -> Traversal {
    use Direction::*;
    let mut traversal: Traversal = [[0; N * N]; Count as usize];

    for i in 0..(Count as usize) {
        traversal[i] = [0; N * N];
    }

    for x in 0..N {
        traversal[Up][0 * N + x] = x;
        traversal[Down][(N - 1) * N + x] = (N - 1) * N + x;
    }

    for y in 0..N {
        traversal[Left][y * N] = y * N;
        traversal[Right][y * N + (N - 1)] = y * N + (N - 1);
    }

    for y in 1..N {
        for x in 0..N {
            traversal[Up][y * N + x] = (y - 1) * N + x;
            traversal[Down][(y - 1) * N + x] = y * N + x;
        }
    }

    for x in 1..N {
        for y in 0..N {
            traversal[Left][y * N + x] = y * N + (x - 1);
            traversal[Right][y * N + (x - 1)] = y * N + x;
        }
    }

    traversal
}

fn solve_one(instructions: &Vec<&str>) -> Result<Vec<usize>, ()> {
    use Direction::*;
    let traversal = build_traversal();

    let mut digit = 5 - 1;
    let mut digits = Vec::new();

    for digit_instruction in instructions {
        for movement in digit_instruction.chars() {
            let index = match movement {
                'U' => Up,
                'D' => Down,
                'L' => Left,
                'R' => Right,
                _ => return Err(()),
            };

            digit = traversal[index][digit];
        }

        digits.push(digit);
    }

    Ok(digits)
}

const M: usize = 5;
const KEYPAD: [[usize; M]; M] = [[0, 0, 1, 0, 0],
                                 [0, 2, 3, 4, 0],
                                 [5, 6, 7, 8, 9],
                                 [0, 10, 11, 12, 0],
                                 [0, 0, 13, 0, 0]];

type Position = (usize, usize);

fn solve_two(instructions: &Vec<&str>) -> Result<Vec<usize>, ()> {
    let mut position: Position = (0, 2);
    let mut digits = Vec::new();
    let mut digit = 0;

    for instruction in instructions {
        for movement in instruction.chars() {
            use std::cmp::{min, max};

            let new_position = match movement {
                'U' => (position.0, max(position.1.saturating_sub(1), 0)),
                'D' => (position.0, min(position.1.saturating_add(1), M - 1)),
                'L' => (max(position.0.saturating_sub(1), 0), position.1),
                'R' => (min(position.0.saturating_add(1), M - 1), position.1),
                _ => return Err(()),
            };

            let new_digit = KEYPAD[new_position.1][new_position.0];
            if new_digit != 0 {
                position = new_position;
                digit = new_digit;
            }
        }

        digits.push(digit);
    }

    Ok(digits)
}

fn format_one(digits: &Vec<usize>) -> String {
    digits.iter().map(|&x| (x + 1).to_string()).collect::<Vec<_>>().join("-")
}

fn format_two(digits: &Vec<usize>) -> String {
    digits.iter().map(|&x| format!("{:X}", x)).collect::<Vec<_>>().join("-")
}

fn main() {
    let instructions =
        include_str!("../input").trim().split('\n').collect::<Vec<_>>();

    solve_one(&instructions)
        .ok()
        .map(|digits| println!("[1] The code is {}.", format_one(&digits)));

    solve_two(&instructions)
        .ok()
        .map(|digits| println!("[2] The code is {}.", format_two(&digits)));
}

#[test]
fn part_one() {
    let instructions = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
    assert_eq!(solve_one(&instructions), Ok(vec![0, 8, 7, 4]));
}

#[test]
fn part_two() {
    let instructions = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
    assert_eq!(solve_two(&instructions), Ok(vec![5, 0xd, 0xb, 3]));
}
