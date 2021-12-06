fn main() {
    let input = include_str!("../assets/input.txt").lines().next().unwrap();
    let result = find_population_at_day(input, 256);
    println!("{}", result);
}

fn find_population_at_day(input: &str, day: u16) -> usize {
    let population = parse_input(input);
    let mut timers = [0usize; 9];
    population.iter().for_each(|x| {
        timers[*x] += 1;
    });

    for _ in 1..=day {
        let gen_zero = timers[0];
        timers.rotate_left(1);
        timers[6] += gen_zero;
    }

    timers.iter().fold(0, |acc, x| acc + x) as usize
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
const TEST_DATA: &'static str = "3,4,3,1,2";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculates_population() {
        assert_eq!(find_population_at_day(TEST_DATA, 18), 26);
        assert_eq!(find_population_at_day(TEST_DATA, 80), 5934);
    }
}
