use std::collections::HashMap;

const N: usize = 8;

type Frequency = HashMap<char, usize>;

fn solve<F>(input: &[&str], mut f: F) -> Option<String>
    where F: FnMut(&Frequency) -> Option<char> {

    let mut message = String::new();
    let mut frequencies: Vec<Frequency> = Vec::new();

    for _ in 0..N {
        frequencies.push(HashMap::new());
    }

    for line in input {
        for (i, c) in line.chars().enumerate() {
            *frequencies[i].entry(c).or_insert(0) += 1;
        }
    }

    for map in frequencies {
        match f(&map) {
            Some(c) => message.push(c),
            _ => return None,
        }
    }

    Some(message)
}

fn solve_one(input: &[&str]) -> Option<String> {
    solve(input, |freq: &Frequency| {
        freq.iter().max_by_key(|&(_, &x)| x).map(|(&k, _)| k)
    })
}

fn solve_two(input: &[&str]) -> Option<String> {
    solve(input, |freq: &Frequency| {
        freq.iter().min_by_key(|&(_, &x)| x).map(|(&k, _)| k)
    })
}

fn main() {
    let input = include_str!("../input").trim().lines().collect::<Vec<_>>();
    if !input.iter().all(|&x| x.len() == N) {
        println!("Malformed input.");
        return;
    }

    solve_one(&input).map(|x| println!("[1] The message is {}.", x));
    solve_two(&input).map(|x| println!("[2] The message is {}.", x));
}
