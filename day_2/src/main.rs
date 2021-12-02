use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./assets/day_2.txt")?;
    let lines = input
        .split('\n')
        .filter(|n| n.len() > 0)
        .collect::<Vec<_>>();

    println!(
        r#"
            Results:
            Day 2 (Part 1): {part_1}
            Day 2 (Part 2): {part_2}
        "#,
        part_1 = calculate_travel(lines.clone()),
        part_2 = calculate_aim(lines.clone())
    );

    Ok(())
}

fn calculate_travel(directions: Vec<&str>) -> usize {
    let mut h = 0;
    let mut d = 0;

    for instruction in directions {
        let mut split = instruction.split(' ');
        let matcher = (
            split.next().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );

        match matcher {
            ("forward", value) => {
                h += value;
            }
            ("up", value) => {
                d -= value;
            }
            ("down", value) => {
                d += value;
            }
            _ => panic!("Uh oh"),
        }
    }

    d * h
}

fn calculate_aim(directions: Vec<&str>) -> usize {
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;

    for instruction in directions {
        let mut split = instruction.split(' ');
        let matcher = (
            split.next().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );

        match matcher {
            ("forward", value) => {
                h += value;
                d += a * value;
            }
            ("up", value) => {
                a -= value;
            }
            ("down", value) => {
                a += value;
            }
            _ => panic!("Uh oh"),
        }
    }

    d * h
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_travel() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let result = calculate_travel(input);

        assert_eq!(result, 150);
    }

    #[test]
    fn it_calculates_aim() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let result = calculate_aim(input);

        assert_eq!(result, 900);
    }
}
