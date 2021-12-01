use anyhow::Result;
use std::{fs, str::FromStr};

/// Read file at a path to a vector, where each line is an item of the
/// vectro.
pub fn read_to_vec<T: FromStr>(path: &str) -> Result<Vec<T>> {
    let result = fs::read_to_string(path)?
        .split('\n')
        .map(|v| v.parse::<T>())
        .flatten()
        .collect::<Vec<_>>();

    Ok(result)
}
