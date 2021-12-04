use std::fs;

pub struct Reader {
    input: String,
}

impl Reader {
    pub fn new(path: &str) -> anyhow::Result<Reader> {
        let input = fs::read_to_string(path)?;

        Ok(Self { input })
    }

    pub fn items(&self) -> Vec<&str> {
        self.input.lines().collect()
    }

    pub fn map_items<T, F>(&self, map: F) -> Vec<T>
    where
        F: Fn(&str) -> T,
    {
        self.input.lines().map(map).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_items() {
        let reader = Reader {
            input: String::from("abc\ndef\nhij"),
        };

        assert_eq!(reader.items(), vec!["abc", "def", "hij"])
    }

    #[test]
    fn it_maps_items() {
        let reader = Reader {
            input: String::from("1\n2\n3"),
        };

        assert_eq!(
            reader.map_items(|item| item.parse::<usize>().unwrap() * 2),
            vec![2, 4, 6]
        )
    }
}
