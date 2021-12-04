#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub result: Option<usize>,
    selected: Vec<(usize, usize)>,
    grid: [[u8; 5]; 5],
}

impl Board {
    pub fn new(grid: [[u8; 5]; 5]) -> Self {
        Self {
            result: None,
            selected: vec![],
            grid,
        }
    }

    pub fn pick(&mut self, number: u8) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                let current = self.grid[x][y];

                if current == number {
                    self.selected.push((x, y));
                    let win = self.check_row(x) || self.check_col(y);
                    if win {
                        let score = self.calculate_score(number);
                        self.result = Some(score);
                    }
                    return win;
                }
            }
        }

        false
    }

    pub fn calculate_score(&self, multiplier: u8) -> usize {
        let mut sum = 0usize;
        for x in 0..5 {
            for y in 0..5 {
                if !self.selected.contains(&(x, y)) {
                    sum += self.grid[x][y] as usize;
                }
            }
        }
        sum * multiplier as usize
    }

    fn check_row(&self, x: usize) -> bool {
        for i in 0..5 {
            if !self.selected.contains(&(x, i)) {
                return false;
            }
        }

        true
    }

    fn check_col(&self, y: usize) -> bool {
        for i in 0..5 {
            if !self.selected.contains(&(i, y)) {
                return false;
            }
        }

        true
    }
}
