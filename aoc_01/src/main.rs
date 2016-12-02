
#![feature(static_in_const)]

use std::collections::HashSet;

#[derive(PartialEq)]
enum PuzzlePart {
    One,
    Two,
}

enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (i32, i32);
type ContinueWalk = bool;

struct Player {
    part: PuzzlePart,
    facing: Direction,
    trace: HashSet<Position>,
    position: Position,
}

impl Player {
    fn new(part: PuzzlePart) -> Self {
        Player {
            // Poor man's strategy pattern.
            part: part,
            trace: HashSet::new(),

            facing: Direction::North,
            position: (0, 0),
        }
    }

    fn turn_right(&mut self) {
        use Direction::*;

        self.facing = match self.facing {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&mut self) {
        for _ in 0..3 {
            self.turn_right()
        }
    }

    fn steps(&mut self, distance: i32) -> ContinueWalk {
        match self.part {
            PuzzlePart::One => self._forward(distance),
            PuzzlePart::Two => return self._trace_steps(distance),
        }

        true
    }

    fn _forward(&mut self, distance: i32) {
        use Direction::*;

        match self.facing {
            North => self.position.1 += distance,
            East => self.position.0 += distance,
            South => self.position.1 -= distance,
            West => self.position.0 -= distance,
        }
    }

    fn _trace_steps(&mut self, distance: i32) -> bool {
        for _ in 0..distance {
            self._forward(1);
            if self.trace.contains(&self.position) {
                return false;
            }

            self.trace.insert(self.position);
        }

        true
    }
}

type Distance = usize;

fn solve(instructions: &Vec<&str>, part: PuzzlePart) -> Result<Distance, ()> {
    let mut player = Player::new(part);

    for i in instructions {
        match i.chars().next() {
            Some('R') => player.turn_right(),
            Some('L') => player.turn_left(),
            _ => return Err(()),
        }

        let steps = i[1..].parse().map_err(|_| ())?;
        let done = !player.steps(steps);

        if done {
            break;
        }
    }

    let distance = player.position.0.abs() + player.position.1.abs();
    Ok(distance as Distance)
}

fn parse(input: &str) -> Vec<&str> {
    input.trim().split(", ").collect()
}

fn main() {
    let instructions = parse(include_str!("../input"));

    solve(&instructions, PuzzlePart::One)
        .ok()
        .map(|x| println!("[1] The shortest distance is {} blocks.", x));

    solve(&instructions, PuzzlePart::Two)
        .ok()
        .map(|x| println!("[2] The shortest distance is {} blocks.", x));
}

#[test]
fn part_one() {
    const TEST_CASES: [(&str, usize); 3] =
        [("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];

    for &(input, output) in &TEST_CASES {
        let instructions = parse(input);
        assert_eq!(solve(&instructions, PuzzlePart::One), Ok(output));
    }
}

#[test]
fn part_two() {
    let instructions = parse("R8, R4, R4, R8");
    assert_eq!(solve(&instructions, PuzzlePart::Two), Ok(4));
}
