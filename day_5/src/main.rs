use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./assets/day_5.txt").unwrap();
    println!("{}", solve(&input));
}

fn parse_lines(input: &str) -> Vec<Vec<(usize, usize)>> {
    let to_usize = |input: &str| input.parse::<usize>().unwrap();
    let split_point = |point: &str| {
        let mut split = point.split(',');
        let x = split.next().map(to_usize).unwrap();
        let y = split.next().map(to_usize).unwrap();
        (x, y)
    };

    input
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|instructions| {
            let mut parts = instructions.split_whitespace();
            let (x1, y1) = parts.nth(0).map(split_point).unwrap();
            let (x2, y2) = parts.nth(1).map(split_point).unwrap();

            let (x_start, x_end) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let (y_start, y_end) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

            let mut xs = (x_start..=x_end).collect::<Vec<_>>();
            if x1 > x2 {
                xs.reverse();
            }
            let mut ys = (y_start..=y_end).collect::<Vec<_>>();
            if y1 > y2 {
                ys.reverse();
            }

            if xs.len() == 1 {
                xs = xs.repeat(ys.len());
            } else if ys.len() == 1 {
                ys = ys.repeat(xs.len());
            }

            xs.into_iter().zip(ys.into_iter()).collect()
        })
        .collect()
}

fn solve(input: &str) -> usize {
    let lines = parse_lines(input);
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    lines.iter().flatten().for_each(|point| {
        if !map.contains_key(&point) {
            map.insert(*point, 1);
        } else {
            let value = *map.get(&point).unwrap();
            map.insert(*point, value + 1);
        }
    });

    let has_two = map.iter().fold(0usize, |acc, (_, total)| {
        if *total >= 2 {
            return acc + 1;
        }
        acc
    });

    has_two
}

#[cfg(test)]
const TEST_INPUT: &'static str = r#"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runs_solve() {
        assert_eq!(solve(TEST_INPUT), 12);
    }
}
