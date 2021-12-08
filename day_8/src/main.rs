use std::collections::HashSet;

fn main() {
    let input = include_str!("../assets/input.txt");
    println!("Part I: {}", star_one(input));
}

fn star_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let outputs = lines
        .iter()
        .map(|x| x.split(" | ").nth(1).unwrap())
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .flatten()
        .filter(|x| match x.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    outputs.len()
}

fn star_two(input: &str) -> usize {
    let outputs: HashSet<&str> = input
        .lines()
        .map(|x| {
            x.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect();

    println!("{:?}", outputs);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../assets/sample.txt");

    #[test]
    fn solve_one() {
        assert_eq!(star_one(SAMPLE), 26);
    }

    #[test]
    fn solve_two() {
        assert_eq!(star_two(SAMPLE), 61229);
    }
}
