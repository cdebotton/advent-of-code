fn main() {
    let input = include_str!("../assets/input.txt");

    println!("I: {}", solve_one(input));
    println!("II: {}", solve_two(input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve_one(input: &str) -> i32 {
    let mut numbers = parse_input(input);

    numbers.sort();
    let mid = numbers.len() / 2;
    let median = numbers[mid] as i32;

    let result = numbers
        .iter()
        .fold(0, |acc, x| acc + (*x as i32 - median).abs());

    result as i32
}

fn solve_two(input: &str) -> i32 {
    let calc_fuel = |distance: i32| {
        let mut cost = 0;
        for i in 0..=distance {
            cost += i;
        }

        cost
    };

    let numbers = parse_input(input);
    let avg = numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
    let ceil = numbers
        .iter()
        .map(|x| {
            let d = (*x - avg.ceil() as i32).abs();
            calc_fuel(d)
        })
        .sum::<i32>();
    let floor = numbers
        .iter()
        .map(|x| {
            let d = (*x - avg.floor() as i32).abs();
            calc_fuel(d)
        })
        .sum::<i32>();

    floor.min(ceil)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_one() {
        let input = include_str!("../assets/example.txt");
        assert_eq!(solve_one(input), 37);
    }

    #[test]
    fn star_two() {
        let input = include_str!("../assets/example.txt");
        assert_eq!(solve_two(input), 168);
    }
}
