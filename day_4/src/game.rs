use std::collections::VecDeque;

use crate::board::Board;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub picked: Vec<u8>,
    winners: Vec<Board>,
    boards: Vec<Board>,
}

impl Game {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines().collect::<VecDeque<_>>();

        let picked: Vec<u8> = if let Some(first) = lines.pop_front() {
            first.split(',').map(|x| x.parse().unwrap()).collect()
        } else {
            vec![]
        };

        let mut grids: Vec<[[u8; 5]; 5]> = vec![];
        let mut current_row = 0;

        while let Some(line) = lines.pop_front() {
            if line == "" {
                grids.push([[0; 5]; 5]);
                current_row = 0;
                continue;
            }

            if let Some(grid) = grids.last_mut() {
                let values: Vec<u8> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                for i in 0..5 {
                    grid[current_row][i] = values[i];
                }
            }

            current_row += 1;
        }

        let boards = grids.iter().map(|x| Board::new(*x)).collect();

        Self {
            picked,
            boards,
            winners: vec![],
        }
    }

    pub fn run(&mut self) -> &Vec<Board> {
        for x in &self.picked {
            self.boards.iter_mut().for_each(|board| {
                if None == board.result {
                    if board.pick(*x) {
                        self.winners.push(board.clone());
                    }
                }
            });
        }

        &self.winners
    }
}
