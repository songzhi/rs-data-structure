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
    let mut row_min_set = HashSet::new();
    let mut col_max_set = HashSet::new();
    for (i, row) in matrix.iter().enumerate() {
        let (j, _) = row.iter().enumerate().min_by_key(|(i, &elem)| elem).unwrap();
        row_min_set.insert(Pos(i, j));
    }
    let matrix_len = matrix.len();
    for j in 0..matrix_len {
        let mut max = &matrix[0][j];
        let mut max_index = 0;
        for i in 1..matrix_len {
            if matrix[i][j] > *max {
                max = &matrix[i][j];
                max_index = i;
            }
        }
        col_max_set.insert(Pos(max_index, j));
    }
    row_min_set.intersection(&col_max_set).map(|&Pos(x, y)| (x, y)).collect()
}