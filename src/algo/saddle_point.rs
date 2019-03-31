use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct Pos(usize, usize);

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(&[self.0, self.1], state);
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Pos {}

pub fn saddle_points<T: Ord>(matrix: &Vec<Vec<T>>) -> Vec<(usize, usize)> {
    let row_mins: Vec<(usize, usize)> = matrix.iter().enumerate().map(|(i, row)| {
        (i, row.iter().enumerate().min_by_key(|&(_, elem)| elem).unwrap().0)
    }).collect();
    let is_max_in_col = |&(x, y): &(usize, usize)| {
        for i in 0..matrix.len() {
            if matrix[i][y] > matrix[x][y] {
                return false;
            }
        }
        true
    };
    row_mins.into_iter().filter(is_max_in_col).collect()
}