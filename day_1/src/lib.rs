pub fn count_measurement_increases(input: Vec<u16>) -> u16 {
    let mut cursor: (u16, Option<u16>) = (0, None);

    for value in input {
        let (mut count, prev) = cursor;

        if let Some(last_value) = prev {
            if value > last_value {
                count += 1;
            }
        }

        cursor = (count, Some(value));
    }

    cursor.0
}

pub fn count_three_measurement_increases(input: Vec<u16>) -> u16 {
    let mut sums: Vec<u16> = vec![];

    for i in 2..input.len() {
        sums.push(input[i] + input[i - 1] + input[i - 2]);
    }

    count_measurement_increases(sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_increments() {
        let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_measurement_increases(test_data), 7);
    }

    #[test]
    fn it_counts_three_measurements() {
        let test_data = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(count_three_measurement_increases(test_data), 5);
    }
}
