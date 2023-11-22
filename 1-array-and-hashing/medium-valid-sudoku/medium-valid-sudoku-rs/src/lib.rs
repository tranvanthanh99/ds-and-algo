pub struct Solution {}

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hash_map = HashMap::<[u8; 3], HashSet<char>>::new();

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[x][y] != '.' {
                    let existed_in_row = hash_map
                        .entry([1 + x as u8, 0, 0])
                        .or_default()
                        .insert(board[x][y]);
                    let existed_in_column = hash_map
                        .entry([0, 1 + y as u8, 0])
                        .or_default()
                        .insert(board[x][y]);
                    let existed_in_box = hash_map
                        .entry([(x / 3) as u8, (y / 3) as u8, 1 + ((x / 3) + (y / 3)) as u8])
                        .or_default()
                        .insert(board[x][y]);

                    if !existed_in_row || !existed_in_column || !existed_in_box {
                        return false;
                    }
                }
            }
        }

        true
    }
}
