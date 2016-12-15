
extern crate itertools;

use itertools::Itertools;

type Input = Vec<Vec<u32>>;

type Triangle = [u32; 3];
type Triangles = Vec<Triangle>;

fn count_valid_triangles(triangles: &Triangles) -> usize {
    let mut valid = 0;
    for triangle in triangles {
        if triangle[0] + triangle[1] > triangle[2] {
            valid += 1;
        }
    }

    valid
}

fn parse_one(input: &Input) -> Result<Triangles, ()> {
    let mut result = Vec::new();

    for sides in input {
        if sides.len() != 3 {
            return Err(());
        }

        let mut triangle = [0; 3];
        triangle.clone_from_slice(&sides);
        triangle.sort();

        result.push(triangle);
    }

    Ok(result)
}

fn parse_two(input: &Input) -> Result<Triangles, ()> {
    let flat: Vec<u32> = input.into_iter().cloned().flat_map(|x| x).collect();

    let a = flat.iter().step(3);
    let b = flat.iter().dropping(1).step(3);
    let c = flat.iter().dropping(2).step(3);

    let transposed: Vec<_> = a.chain(b.chain(c)).cloned().collect();
    let input = transposed.chunks(3).map(|x| x.to_vec()).collect();

    parse_one(&input)
}

fn solve<F>(input: &Input, parser: F, part: usize)
    where F: Fn(&Input) -> Result<Triangles, ()> {

    parser(&input).ok().map(|t| {
        println!("[{}] There are {} valid triangles.",
                 part,
                 count_valid_triangles(&t))
    });
}

fn main() {
    let input: Input = include_str!("../input")
        .trim()
        .split('\n')
        .map(|x| x.split(' ').filter_map(|x| x.parse().ok()).collect())
        .collect();

    solve(&input, parse_one, 1);
    solve(&input, parse_two, 2);
}
