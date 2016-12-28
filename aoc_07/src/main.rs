use std::collections::HashSet;

enum Sequence {
    Supernet(String),
    Hypernet(String),
}

type Address = Vec<Sequence>;

fn is_palindrome(input: &str) -> bool {
    let half = input.len() / 2;

    let a = input.chars().take(half);
    let b = input.chars().rev().take(half);

    for (x, y) in a.zip(b) {
        if x != y {
            return false;
        }
    }

    true
}

fn parse_address(address: &str) -> Address {
    use Sequence::*;

    let mut result = Vec::new();
    let mut cursor = 0;

    for (index, match_) in address.match_indices(|c| "[]".contains(c)) {
        if cursor != index {
            let current = address[cursor..index].to_string();
            result.push(if match_ == "[" {
                Supernet(current)
            } else {
                Hypernet(current)
            });
        }

        cursor = index + match_.len();
    }

    if cursor != address.len() {
        result.push(Supernet(address[cursor..].to_string()));
    }

    result
}

fn contains_abba(content: &str) -> bool {
    for i in 0..(content.len() - 4 + 1) {
        let a = content.chars().nth(i).unwrap();
        let b = content.chars().nth(i + 1).unwrap();

        if a != b && is_palindrome(&content[i..(i + 4)]) {
            return true;
        }
    }

    false
}

fn collect_aba(content: &str) -> HashSet<(char, char)> {
    let mut result = HashSet::new();
    for i in 0..(content.len() - 3 + 1) {
        let a = content.chars().nth(i).unwrap();
        let b = content.chars().nth(i + 1).unwrap();
        let c = content.chars().nth(i + 2).unwrap();

        if a == c && a != b {
            result.insert((a, b));
        }
    }

    result
}

fn match_bab(content: &str, aba_occurrences: &HashSet<(char, char)>) -> bool {
    for i in 0..(content.len() - 3 + 1) {
        let a = content.chars().nth(i).unwrap();
        let b = content.chars().nth(i + 1).unwrap();
        let c = content.chars().nth(i + 2).unwrap();

        if a == c && a != b && aba_occurrences.contains(&(b, a)) {
            return true;
        }
    }

    false
}

fn supports_tls(address: &[Sequence]) -> bool {
    use Sequence::*;

    let mut any_abba = false;
    for sequence in address {
        match *sequence {
            Supernet(ref content) => {
                any_abba = any_abba || contains_abba(content);
            }

            Hypernet(ref content) => {
                if contains_abba(content) {
                    return false;
                }
            }
        }
    }

    any_abba
}

fn supports_ssl(address: &[Sequence]) -> bool {
    use Sequence::*;

    let collected = address.iter()
        .fold(HashSet::new(),
              |mut acc, x| if let Supernet(ref content) = *x {
                  acc.extend(collect_aba(content));
                  acc
              } else {
                  acc
              });

    address.iter().any(|x| if let Hypernet(ref content) = *x {
        match_bab(content, &collected)
    } else {
        false
    })
}

fn main() {
    let input = include_str!("../input").trim().lines().collect::<Vec<_>>();
    let (mut tls_counter, mut ssl_counter) = (0, 0);

    for line in input {
        let address = parse_address(line);
        if supports_tls(&address) {
            tls_counter += 1;
        }

        if supports_ssl(&address) {
            ssl_counter += 1;
        }
    }

    println!("[1] {} addresses support TLS.", tls_counter);
    println!("[2] {} addresses support SSL.", ssl_counter);
}
