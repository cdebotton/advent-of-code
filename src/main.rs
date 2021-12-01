use anyhow::Result;
use std::fs;

fn read_input_array(path: &str) -> Result<Vec<u16>> {
    let result = fs::read_to_string(path)?
        .split('\n')
        .map(|v| v.parse::<u16>())
        .flatten()
        .collect::<Vec<_>>();

    Ok(result)
}

fn main() -> Result<()> {
    let input_data = read_input_array("assets/day_1.txt")?;
    // let result_day_1 = day_1::count_measurement_increases(input_data);
    let result_day_1_2 = day_1::count_three_measurement_increases(input_data);

    println!("{}", result_day_1_2);

    Ok(())
}
