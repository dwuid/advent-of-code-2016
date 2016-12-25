
#[macro_use]
extern crate lazy_static;
extern crate md5;

use md5::compute;

fn next_pair(input: &[u8], pepper: &mut u64) -> Option<(u8, u8)> {
    let mut preimage = Vec::new();

    // Ignore edge case where maximum pepper yields the digit.
    while *pepper != u64::max_value() {
        preimage.clear();

        preimage.extend_from_slice(input);
        preimage.extend(pepper.to_string().bytes());

        let digest = compute(&preimage);
        let mut occurrence =
            digest.iter().enumerate().skip_while(|&(_, &b)| b == 0);

        *pepper = pepper.saturating_add(1);
        if let Some((index, byte)) = occurrence.next() {
            if (index > 2) || (index == 2 && (byte & 0xf0) == 0) {
                let d6 = digest[2] & 0xf;
                let d7 = (digest[3] >> 4) & 0xf;

                return Some((d6, d7));
            }
        }
    }

    None
}

fn next_digit(input: &[u8], pepper: &mut u64) -> Option<u8> {
    next_pair(input, pepper).map(|(x, _)| x)
}

fn solve_one(input: &[u8]) -> Option<String> {
    let mut password = Vec::new();
    let mut pepper = 0u64;

    for _ in 0..8 {
        match next_digit(input, &mut pepper) {
            Some(digit) => password.push(format!("{:x}", digit)),
            _ => return None,
        }
    }

    Some(password.join(""))
}

fn solve_two(input: &[u8]) -> Option<String> {
    let (mut free, mut pepper) = (8, 0u64);
    let mut password = [!0u8; 8];

    while free != 0 {
        match next_pair(input, &mut pepper) {
            Some((position, digit)) => {
                let position = position as usize;
                if position < 8 && password[position] == !0u8 {
                    free -= 1;
                    password[position] = digit;
                }
            }
            _ => return None,
        }
    }

    Some(password.iter()
        .map(|x| format!("{:x}", x))
        .collect::<Vec<_>>()
        .join(""))
}

fn main() {
    lazy_static! {
        static ref INPUT: Vec<u8> = "ugkcyxxp".bytes().collect();
    }

    match solve_one(&INPUT) {
        Some(password) => println!("[1] Found the password: {}.", password),
        _ => println!("[1] Could not find the password."),
    }

    match solve_two(&INPUT) {
        Some(password) => println!("[2] Found the password: {}.", password),
        _ => println!("[2] Could not find the password."),
    }
}
