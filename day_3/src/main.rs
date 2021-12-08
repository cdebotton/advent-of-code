use std::{fs::read_to_string};

use utils::challenge::Reader;

fn count(nums: &[i32], bit: i32) -> (usize, usize) {
    nums.iter().fold((0, 0), |(z, o), n| {
        if (n & bit) == bit {
            (z, o + 1)
        } else {
            (z + 1, o)
        }
    })
}

fn solve(input: &str) -> i32 {
    let last = input.lines().next().unwrap().len() - 1;
    let nums: Vec<i32> = input
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();
    let mut o_nums = nums.clone();
    let mut c_nums = nums;
    for bit in (0..=last).rev().map(|i| 1 << i) {
        let reduce =
            |nums: Vec<i32>, want| nums.iter().copied().filter(|n| n & bit == want).collect();
        if o_nums.len() > 1 {
            let (z, o) = count(&o_nums, bit);
            o_nums = reduce(o_nums, if o >= z { bit } else { 0 });
        }
        if c_nums.len() > 1 {
            let (z, o) = count(&c_nums, bit);
            c_nums = reduce(c_nums, if o < z { bit } else { 0 });
        }
    }
    o_nums[0] * c_nums[0]
}

fn main() {
    let input = Reader::new("./assets/day_3.txt").unwrap();
    let items = input.items();

    let (gamma, epsilon) = calculate_gamma_epsilon(items.clone());
    let (oxygen, co2) = calculate_oxygen_co2(&gamma, &epsilon, items.clone());

    println!(
        r#"
            Results:
            Day 3 (Part 1): {part_1}
            Day 3 (Part 2): {part_2}
        "#,
        part_1 = to_dec(&gamma) * to_dec(&epsilon),
        part_2 = solve(&read_to_string("./assets/day_3.txt").unwrap())
    );
}

fn to_dec(str: &str) -> usize {
    usize::from_str_radix(&str, 2).unwrap()
}

fn calculate_gamma_epsilon(input: Vec<&str>) -> (String, String) {
    let line_bits = input
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    let len = line_bits[0].len();
    for i in 0..len {
        let mut ones = 0;
        let mut zeroes = 0;
        for line in &line_bits {
            let bit = &line[i];
            if *bit == '0' {
                zeroes += 1
            } else {
                ones += 1
            };
        }

        if ones > zeroes {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    (gamma, epsilon)
}

fn calculate_oxygen_co2(gamma: &str, epsilon: &str, input: Vec<&str>) -> (String, String) {
    let input_count = input.len();
    let mut oxygen_count = 0;
    let mut co2_count = 0;

    let mut oxygen: Option<&str> = None;
    let mut co2: Option<&str> = None;

    for i in 0..gamma.len() {
        for binary in &input {
            let current_char = binary.chars().nth(i).unwrap();

            if current_char != gamma.chars().nth(i).unwrap() {
                if oxygen_count == input_count - 1 {
                    oxygen = Some(binary);
                    break;
                }
                oxygen_count += 1;
            }

            if current_char != epsilon.chars().nth(i).unwrap() {
                if co2_count == input_count - 1 {
                    co2 = Some(binary);
                    break;
                }
                co2_count += 1;
            }
        }
    }

    println!("{:#?}, {:#?}", oxygen, co2);

    (String::new(), String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_rates() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let (gamma, epsilon) = calculate_gamma_epsilon(input);

        assert_eq!(to_dec(&gamma) * to_dec(&epsilon), 198)
    }

    #[test]
    fn it_calculates_oxygen_co2() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let (gamma, epsilon) = calculate_gamma_epsilon(input.clone());
        let (oxygen, co2) = calculate_oxygen_co2(&gamma, &epsilon, input);
        // assert_eq!(to_dec(&oxygen) * to_dec(&co2), 230)
    }
}
